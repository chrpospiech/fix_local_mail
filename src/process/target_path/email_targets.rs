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
/// - `test_keep_name_for_email_in_maildir`: Verifies that existing remote_id
///   is preserved in the new file name for dry-run false.
/// - `test_keep_name_for_email_in_maildir_dry_run`: Verifies that existing remote_id
///   is preserved in the new file name for dry-run true.
/// - `test_handle_odd_email_names_in_maildir`: Verifies that odd email names
///   are correctly processed and a new email name is generated, based on a timestamp
///   extracted from the email content.
/// - `test_handle_odd_email_names_in_maildir_dry_run`: Verifies that odd email names
///   are correctly processed in dry-run mode and a new email name is generated,
///   based on a timestamp extracted from the email content.
/// - `test_get_target_file_name_for_cached`: Verifies target file name generation
///   for emails cached in the database, ensuring timestamps are correctly extracted
///   from the email content (dry_run = false).
/// - `test_get_target_file_name_for_cached_dry_run`: Verifies target file name generation
///   for emails cached in the database in dry-run mode, ensuring timestamps are correctly
///   extracted from the email content (dry_run = true).
/// - `test_get_target_file_name_for_stored_mail`: Verifies target file name generation
///   for emails stored in mail cache, ensuring timestamps are correctly extracted from
///   the email content without dry-run mode.
/// - `test_get_target_file_name_for_stored_mail_dry_run`: Verifies target file name generation
///   for emails stored in mail cache, ensuring timestamps are correctly extracted from
///   the email content in dry-run mode.
mod tests {

    use crate::cmdline::CliArgs;
    use crate::mockup::{create_test_cli_args, setup_tmp_mail_dir, teardown_tmp_mail_dir};
    use crate::process::{
        maildirs::fetch_full_paths, source_path::get_source_file_name,
        target_path::get_target_file_name,
    };
    use crate::todoitems::TodoPimItem;
    use anyhow::Result;
    use regex::Regex;
    use sqlx::mysql::MySqlPool;
    use std::collections::HashMap;

    /// Helper function to perform common assertions on the generated target file name.
    /// The helper function is used in multiple test cases to avoid code duplication.
    /// The helper function first calls the function get_source_file_name to
    /// retrieve the source file name for a given item and then calls get_target_file_name
    /// to generate the target file name. It then performs assertions on the generated target
    /// file name to ensure it meets the expected criteria.
    /// It checks that the file name is not empty, contains the maildir path, does
    /// not contain double slashes, and contains the expected timestamp.
    /// The helper function also checks whether the source file name matches the
    /// regex pattern for a valid email name. If the source file name does match the
    /// pattern, the target file name is expected to contain the same email name with the
    /// "SEEN" flag appended. Otherwise the target file name is expected to contain a new
    /// email name starting with the expected timestamp and with the "SEEN" flag appended.
    /// The helper function is used in the test cases for both dry-run and non-dry-run
    /// scenarios to ensure consistent validation of the target file name generation logic.
    ///
    /// # Arguments
    /// * `pool` - A MySQL connection pool for database access.
    /// * `item` - The TodoPimItem for which the target file name is being generated.
    /// * `full_paths` - A mapping of collection IDs to their corresponding full paths.
    /// * `args` - The CLI arguments containing configuration for the test.
    /// * `expected_timestamp` - The expected timestamp to be included in the target file name.
    ///
    /// Returns a Result indicating success or failure of the assertions.
    ///
    /// # Errors
    /// This function will return an error if any of the assertions fail, such as if the
    /// generated target file name does not meet the expected criteria or if there are issues
    /// with retrieving the source file name or generating the target file name.
    async fn assert_target_file_name_generation(
        pool: MySqlPool,
        item: &TodoPimItem,
        full_paths: &HashMap<i64, String>,
        args: &CliArgs,
        expected_timestamp: i64,
    ) -> Result<()> {
        let source_file_name: Option<String> =
            get_source_file_name(pool.clone(), item, full_paths, args).await?;
        assert!(source_file_name.is_some());
        let source_file_name = source_file_name.unwrap();
        assert!(!source_file_name.is_empty());
        assert!(!source_file_name.contains("//"));

        // Test: Retrieve the target file name for the same file_id
        let target_file_name: String = get_target_file_name(
            pool.clone(),
            item,
            full_paths,
            &source_file_name, // use the extracted time_stamp
        )
        .await?;
        assert!(!target_file_name.is_empty());
        assert!(target_file_name.contains(&args.maildir_path));
        assert!(!target_file_name.contains("//"));
        assert!(!target_file_name.is_empty());
        assert!(target_file_name.contains(&args.maildir_path));
        assert!(!target_file_name.contains("//"));
        // Verify that the remote_id part is preserved in the target file name
        let re = Regex::new(r"(\d+\.R\d+\.\w+)").unwrap();
        if let Some(caps) = re.captures(&source_file_name) {
            assert!(target_file_name.contains(caps.get(1).unwrap().as_str()));
        } else {
            // If the source file name does not match the email name pattern,
            // then the target file name should contain a new email name starting
            // with the expected timestamp.
            assert!(target_file_name.contains(&format!("{}.R", expected_timestamp)));
        }
        // Verify that the "SEEN" flag is appended
        assert!(target_file_name.contains("helios:2,S"));

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_keep_name_for_email_in_maildir(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;
        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;
        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        // We are testing a case where the remote_id can be kept as is.
        let remote_id = "1767111571664.R424.helios".to_string();
        let item = TodoPimItem {
            id: 50628,
            remote_id: Some(remote_id.clone()),
            collection_id: 394,
        };
        let _expected_timestamp: i64 = 1767111571664; // Extracted from email name.
        assert_target_file_name_generation(pool, &item, &full_paths, &args, _expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_keep_name_for_email_in_maildir_dry_run(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;
        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, true);
        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;
        // Test: Retrieve the target file name for a file_id
        // that is stored in tests/data and has a remote_id.
        // We are testing a case where the remote_id can be kept as is.
        let remote_id = "1767111571664.R424.helios".to_string();
        let item = TodoPimItem {
            id: 50628,
            remote_id: Some(remote_id.clone()),
            collection_id: 394,
        };
        let _expected_timestamp: i64 = 1767111571664; // Extracted from email name.
        assert_target_file_name_generation(pool, &item, &full_paths, &args, _expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_handle_odd_email_names_in_maildir(pool: MySqlPool) -> Result<()> {
        // Recursively copy tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the target file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let item = TodoPimItem {
            id: 206,
            remote_id: Some(remote_id.clone()),
            collection_id: 388,
        };
        let expected_timestamp = 1686315625; // Extracted from email content
        assert_target_file_name_generation(pool, &item, &full_paths, &args, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_handle_odd_email_names_in_maildir_dry_run(pool: MySqlPool) -> Result<()> {
        // Recursively copy tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, true);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the target file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let item = TodoPimItem {
            id: 206,
            remote_id: Some(remote_id.clone()),
            collection_id: 388,
        };
        let expected_timestamp = 1686315625; // Extracted from email content
        assert_target_file_name_generation(pool, &item, &full_paths, &args, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_target_file_name_for_cached(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;
        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;
        // Test: Retrieve the target file name for a file_id
        // that is cached in the database and does not have a remote_id.
        // We are testing a case where the email name needs to be generated based on
        // the content of the email.
        let item = TodoPimItem {
            id: 50645,
            remote_id: None,
            collection_id: 394,
        };
        let expected_timestamp = 1767111447; // Extracted from email content
        assert_target_file_name_generation(pool, &item, &full_paths, &args, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_target_file_name_for_cached_dry_run(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;
        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, true);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the target file name for a file_id
        // that is cached in the database and does not have a remote_id.
        // We are testing a case where the email name needs to be generated based on
        // the content of the email.
        let item = TodoPimItem {
            id: 50645,
            remote_id: None,
            collection_id: 394,
        };
        let expected_timestamp = 1767111447; // Extracted from email content
        assert_target_file_name_generation(pool, &item, &full_paths, &args, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_target_file_name_for_stored_mail(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;
        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the target file name for a file_id
        // that is cached on disk.
        // We are testing a case where the email name needs to be generated based on
        // the content of the email.
        let item = TodoPimItem {
            id: 50642,
            remote_id: None,
            collection_id: 394,
        };
        let expected_timestamp = 1767092602; // Extracted from email content
        assert_target_file_name_generation(pool, &item, &full_paths, &args, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_target_file_name_for_stored_mail_dry_run(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;
        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, true);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the target file name for a file_id
        // that is cached on disk.
        // We are testing a case where the email name needs to be generated based on
        // the content of the email.
        let item = TodoPimItem {
            id: 50642,
            remote_id: None,
            collection_id: 394,
        };
        let expected_timestamp = 1767092602; // Extracted from email content
        assert_target_file_name_generation(pool, &item, &full_paths, &args, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }
}
