#[cfg(test)]
/// Test module for email source functionality.
///
/// This module contains integration tests that verify the email retrieval system
/// works correctly for both file-based and database-based email caching scenarios.
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
///   in the filesystem (file_id 50638, no remote_id)
/// - `test_get_cached_email_from_db`: Verifies email retrieval when the email is cached
///   in the database (file_id 50645, has remote_id)
///
/// # Regarding the `.map_err(std::io::Error::other)` call
///
/// The `map` here is used because `fs_extra::dir::copy()` returns a custom error type
/// (`fs_extra::error::Error`), but we need to convert it to `std::io::Error` for
/// compatibility with the `.expect()` call and standard error handling. The `map_err()`
/// function transforms the error type while preserving the error information.
mod tests {

    use crate::{
        cmdline::CliArgs,
        todoitems::maildirs::fetch_full_paths,
        todoitems::source_path::{get_cached_email, get_source_file_name},
    };
    use sqlx::MySqlPool;

    pub fn setup_tmp_mail_dir() -> String {
        // Create a temporary mail directory structure for testing
        // Recursively copy src/todoitems/tests/data to this structure
        let temp_dir = std::env::temp_dir().join(format!("maildir_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).expect("Failed to create temp maildir");
        let manifest_dir = env!("CARGO_MANIFEST_DIR"); // compile-time
        let path = std::path::Path::new(manifest_dir).join("src/todoitems/tests/data");
        let mut options = fs_extra::dir::CopyOptions::new();
        options.content_only = true;
        fs_extra::dir::copy(&path, &temp_dir, &options)
            .map_err(std::io::Error::other)
            .expect("Could not copy mail directories");

        temp_dir.to_string_lossy().to_string()
    }

    pub fn teardown_tmp_mail_dir(temp_dir: &str) {
        std::fs::remove_dir_all(temp_dir).expect("Failed to remove temp maildir");
    }

    pub fn create_test_cli_args(temp_dir: &str, db_url_auto: bool) -> CliArgs {
        CliArgs {
            maildir_path: format!("{}/local_mail/", temp_dir),
            mail_cache_path: format!("{}/file_db_data/", temp_dir),
            db_url: if db_url_auto {
                "auto".to_string()
            } else {
                "some_other_db_url".to_string()
            },
            ..Default::default()
        }
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_cached_email_from_file(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir();

        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, true);

        // Test: Retrieve the cached email path for file_id 50638
        // The email with file_id 50638 has no remote_id and is cached in the file system
        let file_id = 50638;
        let result: String = get_cached_email(file_id, pool.clone(), &args).await;
        assert!(!result.is_empty());
        assert!(result.contains(&args.mail_cache_path));
        assert!(!result.contains("//"));
        assert!(std::path::Path::new(&result).exists());
        assert!(std::path::Path::new(&result).is_file());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir);

        Ok(())
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_cached_email_pattern(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir();

        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, false);

        // Test: Retrieve the cached email path for file_id 50638
        // The email with file_id 50638 has no remote_id and is cached in the file system
        // We simulate a non-auto db_url to test that no pattern matching occurs
        let file_id = 50638;
        let result: String = get_cached_email(file_id, pool.clone(), &args).await;
        assert!(!result.is_empty());
        assert!(result.contains(&args.mail_cache_path));
        assert!(!result.contains("//"));
        assert!(result.contains("*"));
        assert!(!std::path::Path::new(&result).exists());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir);

        Ok(())
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_cached_email_from_db(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir();

        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, true);

        // Test: Retrieve the cached email path for file_id 50645
        // The email with file_id 50645 has no remote_id and is cached in the database
        // This should create a temporary file with the email contents
        let file_id = 50645;
        let result: String = get_cached_email(file_id, pool.clone(), &args).await;
        assert!(!result.is_empty());
        assert!(!result.contains("//"));
        assert!(result.contains(&args.mail_cache_path));
        assert!(std::path::Path::new(&result).exists());
        assert!(std::path::Path::new(&result).is_file());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir);

        Ok(())
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_not_caching_email(pool: MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir();

        // Setup an argument struct
        let args = create_test_cli_args(&temp_dir, false);

        // Test: Retrieve the cached email path for file_id 50645
        // The email with file_id 50645 has no remote_id and is cached in the database
        // However, for db_url != auto, this should not create a temporary file
        let file_id = 50645;
        let result: String = get_cached_email(file_id, pool.clone(), &args).await;
        assert!(!result.is_empty());
        assert!(!result.contains("//"));
        assert!(result.contains(&args.mail_cache_path));
        assert!(!std::path::Path::new(&result).exists());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir);

        Ok(())
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_source_file_name_with_auto_db(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir();

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, true);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await;

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let file_id = 206;
        let collection_id = 388;
        let path = full_paths
            .get(&collection_id)
            .cloned()
            .unwrap_or("tbd/".to_string());
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let result: String =
            get_source_file_name(path, Some(&remote_id), file_id, pool.clone(), &args).await;
        assert!(!result.is_empty());
        assert!(result.contains(&args.maildir_path));
        assert!(result.contains(&remote_id));
        assert!(!result.contains("//"));
        assert!(std::path::Path::new(&result).exists());
        assert!(std::path::Path::new(&result).is_file());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir);

        Ok(())
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_pattern_for_source_file_name(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir();

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await;

        // Test: Retrieve the source file name for a file_id
        // that is stored in tests/data and has a remote_id.
        let file_id = 206;
        let collection_id = 388;
        let path = full_paths
            .get(&collection_id)
            .cloned()
            .unwrap_or("tbd/".to_string());
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let result: String =
            get_source_file_name(path, Some(&remote_id), file_id, pool.clone(), &args).await;
        assert!(!result.is_empty());
        assert!(result.contains(&args.maildir_path));
        assert!(result.contains(&remote_id));
        assert!(!result.contains("//"));
        assert!(result.contains("*"));
        assert!(!std::path::Path::new(&result).exists());

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir);

        Ok(())
    }
}
