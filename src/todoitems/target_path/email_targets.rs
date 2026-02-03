// Copyright 2026 fix_local_mail C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
/// Test module for email target functionality.
///
/// This module contains integration tests that verify target path/name generation
/// works correctly for both file-based and database-based email caching scenarios.
/// works correctly for both emails stored in mail directories and in the database.
///
/// # Test Setup
///
/// The tests create a temporary mail directory structure by copying test data from
/// `src/todoitems/tests/data` to a unique temporary location. This ensures test
/// isolation and prevents interference between test runs.
///
/// # Helper Functions
///
/// - `setup_tmp_mail_dir()`: Creates a temporary maildir with test data copied from fixtures
/// - `teardown_tmp_mail_dir()`: Cleans up the temporary directory after tests complete
/// - `create_test_cli_args()`: Constructs CLI arguments pointing to the temporary test directories
///
/// # Test Cases
///
/// - `test_get_target_file_name_for_stored_email`: Verifies target file name generation
///   for emails with existing remote_id in the database, ensuring timestamps are read
///   from the remote_id rather than generated fresh.
/// - `test_get_fake_target_file_name_for_stored_email`: Verifies target file name generation
///   when db_url != "auto", ensuring new timestamps are generated instead of reading
///   from remote_id.
/// - `test_get_old_target_file_name_for_stored_email`: Verifies that existing remote_id
///   is preserved in the new file name.
mod tests {

    use crate::cmdline::CliArgs;
    use crate::mockup::{create_test_cli_args, setup_tmp_mail_dir, teardown_tmp_mail_dir};
    use crate::todoitems::{
        maildirs::fetch_full_paths,
        source_path::get_source_file_name,
        target_path::{get_mail_time_stamp, get_target_file_name, get_time_now_secs},
    };
    use anyhow::Result;
    use sqlx::mysql::MySqlPool;

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_target_file_name_for_stored_email(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, true);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;
        // Get current time in seconds minus half an hour
        let recent_secs: u64 = get_time_now_secs()?.saturating_sub(1800);

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let file_id = 206;
        let collection_id = 388;
        let path = full_paths
            .get(&collection_id)
            .cloned()
            .unwrap_or("tbd/".to_string());
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let source_file_name: Option<String> =
            get_source_file_name(path.clone(), Some(&remote_id), file_id, pool.clone(), &args)
                .await?;
        assert!(source_file_name.is_some());
        let source_file_name = source_file_name.unwrap();
        assert!(!source_file_name.is_empty());
        assert!(source_file_name.contains(&args.maildir_path));
        assert!(source_file_name.contains(&remote_id));
        assert!(!source_file_name.contains("//"));
        assert!(std::path::Path::new(&source_file_name).exists());
        assert!(std::path::Path::new(&source_file_name).is_file());

        // Test: Retrieve the target file name for the same file_id
        let target_file_name: String = get_target_file_name(
            path,
            Some(&remote_id),
            source_file_name.clone(),
            file_id,
            pool.clone(),
            &args,
        )
        .await?;
        assert!(!target_file_name.is_empty());
        assert!(target_file_name.contains(&args.maildir_path));
        assert!(!target_file_name.contains("//"));
        // Verify that the "SEEN" flag is appended to the target file name
        assert!(target_file_name.contains(":2,S"));
        // Extract timestamp from target file name
        // This also indirectly verifies that the target file name was generated correctly
        let re = regex::Regex::new(r"(\d+)\.R\d+\.\w+").unwrap();
        let caps = re.captures(&target_file_name).unwrap();
        let timestamp_str = caps.get(1).unwrap().as_str();
        let timestamp: u64 = timestamp_str.parse().unwrap();
        // Verify timestamp is sufficiently old
        // (i.e., was read from remote_id, not newly generated now)
        // This is where we need the source file to exist with the correct remote_id
        assert!(timestamp <= recent_secs);
        assert!(recent_secs - timestamp > 7200);
        let expected_timestamp = get_mail_time_stamp(&source_file_name, &args)?;
        assert_eq!(timestamp, expected_timestamp);

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_fake_target_file_name_for_stored_email(pool: MySqlPool) -> Result<()> {
        // Setup an argument struct with db_url != "auto"
        // to trigger fake target name generation
        // (i.e., not reading from remote_id)
        // We don't create a temporary maildir for this test case,
        // since the source file existence is not required.
        let temp_dir: String = "some_non_existing_path".to_string();
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Get current time in seconds minus half an hour
        let recent_secs: u64 = get_time_now_secs()?.saturating_sub(1800);

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let file_id = 206;
        let collection_id = 388;
        let path = full_paths
            .get(&collection_id)
            .cloned()
            .unwrap_or("tbd/".to_string());
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        // Even though we provide a remote_id, since db_url != "auto",
        // the source file name will be generated without relying on it.
        // It is only a regex pattern in this case and contains a "*".
        let source_file_name: Option<String> =
            get_source_file_name(path.clone(), Some(&remote_id), file_id, pool.clone(), &args)
                .await?;
        assert!(source_file_name.is_some());
        let source_file_name = source_file_name.unwrap();
        assert!(!source_file_name.is_empty());
        assert!(source_file_name.contains(&args.maildir_path));
        assert!(source_file_name.contains(&remote_id));
        assert!(!source_file_name.contains("//"));
        assert!(source_file_name.contains("*"));

        // Test: Retrieve the target file name for the same file_id
        let target_file_name: String = get_target_file_name(
            path,
            Some(&remote_id),
            source_file_name.clone(),
            file_id,
            pool.clone(),
            &args,
        )
        .await?;
        assert!(!target_file_name.is_empty());
        assert!(target_file_name.contains(&args.maildir_path));
        assert!(!target_file_name.contains("//"));
        // Verify that the "SEEN" flag is appended to the target file name
        assert!(target_file_name.contains(":2,S"));
        // Extract timestamp from target file name
        // This also indirectly verifies that the target file name was generated correctly
        let re = regex::Regex::new(r"(\d+)\.R\d+\.\w+").unwrap();
        let caps = re.captures(&target_file_name).unwrap();
        let timestamp_str = caps.get(1).unwrap().as_str();
        let timestamp: u64 = timestamp_str.parse().unwrap();
        // Verify timestamp is sufficiently new
        // (i.e., was not read from remote_id, but newly generated now)
        assert!(timestamp >= recent_secs);
        assert!(timestamp - recent_secs < 1800);

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_old_target_file_name_for_stored_email(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Setup an argument struct with db_url != "auto"
        // to trigger fake target name generation
        // (i.e., not reading from remote_id)
        // We don't create a temporary maildir for this test case,
        // since the source file existence is not required.
        let temp_dir: String = "some_non_existing_path".to_string();
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        // We are testing a case where the remote_id can be kept as is.
        let file_id = 50628;
        let collection_id = 394;
        let path = full_paths
            .get(&collection_id)
            .cloned()
            .unwrap_or("tbd/".to_string());
        let remote_id = "1767111571664.R424.helios".to_string();
        // Even though we provide a remote_id, since db_url != "auto",
        // the source file name will be generated without relying on it.
        // It is only a regex pattern in this case and contains a "*".
        let source_file_name: Option<String> =
            get_source_file_name(path.clone(), Some(&remote_id), file_id, pool.clone(), &args)
                .await?;
        assert!(source_file_name.is_some());
        let source_file_name = source_file_name.unwrap();
        assert!(!source_file_name.is_empty());
        assert!(source_file_name.contains(&args.maildir_path));
        assert!(source_file_name.contains(&remote_id));
        assert!(!source_file_name.contains("//"));
        assert!(source_file_name.contains("*"));

        // Test: Retrieve the target file name for the same file_id
        let target_file_name: String = get_target_file_name(
            path,
            Some(&remote_id),
            source_file_name.clone(),
            file_id,
            pool.clone(),
            &args,
        )
        .await?;
        assert!(!target_file_name.is_empty());
        assert!(target_file_name.contains(&args.maildir_path));
        assert!(!target_file_name.contains("//"));
        // Verify that the remote_id part is preserved in the target file name
        // and the "SEEN" flag is appended
        assert!(target_file_name.contains("1767111571664.R424.helios:2,S"));

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_fake_target_file_name_for_cached_dry_run(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto" and dry_run = true
        let args = CliArgs {
            maildir_path: format!("{}/local_mail/", temp_dir),
            mail_cache_path: format!("{}/file_db_data/", temp_dir),
            db_url: "auto".to_string(),
            dry_run: true,
            ..Default::default()
        };

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Get current time in seconds minus half an hour
        let recent_secs: u64 = get_time_now_secs()?.saturating_sub(1800);

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let file_id = 50645;
        let collection_id = 394;
        let path = full_paths
            .get(&collection_id)
            .cloned()
            .unwrap_or("tbd/".to_string());
        let source_file_name: Option<String> =
            get_source_file_name(path.clone(), None, file_id, pool.clone(), &args).await?;
        assert!(source_file_name.is_some());
        let source_file_name = source_file_name.unwrap();
        assert!(!source_file_name.is_empty());
        assert!(source_file_name.contains(&args.mail_cache_path));
        assert!(!source_file_name.contains("//"));
        assert!(!std::path::Path::new(&source_file_name).exists());

        // Test: Retrieve the target file name for the same file_id
        let target_file_name: String = get_target_file_name(
            path,
            None,
            source_file_name.clone(),
            file_id,
            pool.clone(),
            &args,
        )
        .await?;
        assert!(!target_file_name.is_empty());
        assert!(target_file_name.contains(&args.maildir_path));
        assert!(!target_file_name.contains("//"));
        // Verify that the "SEEN" flag is appended to the target file name
        assert!(target_file_name.contains(":2,S"));
        // Extract timestamp from target file name
        // This also indirectly verifies that the target file name was generated correctly
        let re = regex::Regex::new(r"(\d+)\.R\d+\.\w+").unwrap();
        let caps = re.captures(&target_file_name).unwrap();
        let timestamp_str = caps.get(1).unwrap().as_str();
        let timestamp: u64 = timestamp_str.parse().unwrap();
        // Verify timestamp is sufficiently new
        // (i.e., was not read from remote_id, but newly generated now)
        assert!(timestamp >= recent_secs);
        assert!(timestamp - recent_secs < 1800);

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }
}
