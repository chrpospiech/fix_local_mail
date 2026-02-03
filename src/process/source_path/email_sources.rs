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
/// Test module for email source functionality.
///
/// This module contains integration tests that verify the email retrieval system
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
/// - `test_get_cached_email_from_file`: Verifies email retrieval when the email is cached
///   in the filesystem (file_id 50638, no remote_id) with db_url set to "auto"
/// - `test_get_cached_email_pattern`: Verifies that when db_url is not "auto", the cached
///   email path contains a wildcard pattern instead of being resolved to an actual file
/// - `test_get_cached_email_from_db`: Verifies email retrieval when the email is cached
///   in the database (file_id 50645, has remote_id) and creates a temporary file with db_url="auto"
/// - `test_not_caching_email`: Verifies that when db_url is not "auto", emails cached in the
///   database return a path pattern without creating a temporary file
/// - `test_get_source_file_name_with_auto_db`: Verifies source file name resolution for an
///   email with a remote_id when db_url is "auto"
/// - `test_get_pattern_for_source_file_name`: Verifies source file name returns a wildcard
///   pattern when db_url is not "auto"
///   in the filesystem (file_id 50638, no remote_id)
/// - `test_get_cached_email_from_db`: Verifies email retrieval when the email is cached
///   in the database (file_id 50645, has remote_id)
///
mod tests {

    use crate::{
        mockup::{create_test_cli_args, setup_tmp_mail_dir, teardown_tmp_mail_dir},
        process::maildirs::fetch_full_paths,
        process::source_path::{get_cached_email, get_source_file_name},
        todoitems::TodoPimItem,
    };
    use anyhow::Result;
    use sqlx::mysql::MySqlPool;

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_cached_email_from_file(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;
        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, true);

        // Test: Retrieve the cached email path for file_id 50638
        // The email with file_id 50638 has no remote_id and is cached in the file system
        let file_id = 50638;
        let result: Option<String> = get_cached_email(file_id, pool.clone(), &args).await?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(!result.is_empty());
        assert!(result.contains(&args.mail_cache_path));
        assert!(!result.contains("//"));
        assert!(std::path::Path::new(&result).exists());
        assert!(std::path::Path::new(&result).is_file());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_cached_email_pattern(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, false);

        // Test: Retrieve the cached email path for file_id 50638
        // The email with file_id 50638 has no remote_id and is cached in the file system
        // We simulate a non-auto db_url to test that no pattern matching occurs
        let file_id = 50638;
        let result: Option<String> = get_cached_email(file_id, pool.clone(), &args).await?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(!result.is_empty());
        assert!(result.contains(&args.mail_cache_path));
        assert!(!result.contains("//"));
        assert!(result.contains("*"));
        assert!(!std::path::Path::new(&result).exists());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_cached_email_from_db(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, true);

        // Test: Retrieve the cached email path for file_id 50645
        // The email with file_id 50645 has no remote_id and is cached in the database
        // This should create a temporary file with the email contents
        let file_id = 50645;
        let result: Option<String> = get_cached_email(file_id, pool.clone(), &args).await?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(!result.is_empty());
        assert!(!result.contains("//"));
        assert!(result.contains(&args.mail_cache_path));
        assert!(std::path::Path::new(&result).exists());
        assert!(std::path::Path::new(&result).is_file());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_not_caching_email(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, false);

        // Test: Retrieve the cached email path for file_id 50645
        // The email with file_id 50645 has no remote_id and is cached in the database
        // However, for db_url != auto, this should not create a temporary file
        let file_id = 50645;
        let result: Option<String> = get_cached_email(file_id, pool.clone(), &args).await?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(!result.is_empty());
        assert!(!result.contains("//"));
        assert!(result.contains(&args.mail_cache_path));
        assert!(!std::path::Path::new(&result).exists());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_source_file_name_with_auto_db(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, true);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let item = TodoPimItem {
            id: 206,
            remote_id: Some(remote_id.clone()),
            collection_id: 388,
        };
        let result: Option<String> =
            get_source_file_name(pool.clone(), &item, &full_paths, &args).await?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(!result.is_empty());
        assert!(result.contains(&args.maildir_path));
        assert!(result.contains(&remote_id));
        assert!(!result.contains("//"));
        assert!(std::path::Path::new(&result).exists());
        assert!(std::path::Path::new(&result).is_file());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_pattern_for_source_file_name(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let item = TodoPimItem {
            id: 206,
            remote_id: Some(remote_id.clone()),
            collection_id: 388,
        };
        let result: Option<String> =
            get_source_file_name(pool.clone(), &item, &full_paths, &args).await?;
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(!result.is_empty());
        assert!(result.contains(&args.maildir_path));
        assert!(result.contains(&remote_id));
        assert!(!result.contains("//"));
        assert!(result.contains("*"));
        assert!(!std::path::Path::new(&result).exists());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }
}
