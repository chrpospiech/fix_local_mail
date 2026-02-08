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
/// Test module for functionality of processing todo pim items
///
/// This module contains unit tests for final email processing.
/// It tests that emails are correctly moved to target maildirs
/// and that the Akonadi database is updated accordingly.
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
/// - `test_moving_email_with_odd_name()`: Tests that an email with a specific remote ID is correctly
///   moved to the target maildir and that the corresponding item is removed from the database. It
///   verifies that the email file exists in the target maildir and that the item ID is cleared
///   from the database.
/// - `test_email_already_in_right_place()`: Tests that an email already in the correct location is not
///   moved and that the item remains in the database. It verifies that the email file still exists in
///   its original location and that the item ID is still present in the database.
/// - `test_email_file_missing()`: Tests that when the email file is missing, the item is removed
///   from the database. It verifies that the item ID is cleared from the database.
/// - `test_dry_run_flag()`: Tests that when the `--dry-run` flag is set, no emails are moved
///   and no items are removed from the database. It verifies that the email files remain in
///   their original locations and that the item IDs are still present in the database.
/// - `test_email_in_new_directory()`: Tests that an email in the "new" directory is moved to "cur"
///   and the item is removed from the database. It verifies that the email file exists in "cur"
///   and that the item ID is cleared from the database.
/// - `test_email_in_file_db_cache()`: Tests that an email stored in the file_db cache directory
///   is moved to the target maildir and the item is removed from the database. It verifies that
///   the email file exists in the target maildir and that the item ID is cleared from the database.
/// - `test_email_in_database()`: Tests that an email stored in the database is moved to the target
///   maildir and the item is removed from the database. It verifies that the email file exists
///   in the target maildir and that the item ID is cleared from the database.
///
mod tests {
    use crate::mockup::{create_test_cli_args, setup_tmp_mail_dir, teardown_tmp_mail_dir};
    use crate::process::{
        maildirs::fetch_full_paths, process_single_todo_item, source_path::get_single_matching_file,
    };
    use crate::todoitems::TodoPimItem;
    use anyhow::Result;
    use sqlx::{MySql, Pool};
    use std::collections::HashMap;

    /// Helper function to perform common assertions on the result of processing a todo item.
    /// This function can be used to verify that the email file has been moved to the correct
    /// target maildir and that the item has been removed from the database.
    /// It takes the database pool, the processed item, the expected mail directory path,
    /// and the expected timestamp as arguments.
    /// It asserts that the email file exists in the target maildir and that the item has been
    /// removed from the database.
    ///
    /// # Arguments
    /// - `pool`: The database connection pool to use for verifying database changes.
    /// - `item`: The `TodoPimItem` that was processed, containing the item ID and collection ID.
    /// - `full_paths`: A mapping of collection IDs to their corresponding full mail directory paths.
    /// - `expected_timestamp`: The expected timestamp extracted from the email content.
    ///
    /// # Returns
    /// - `Result<()>`: Returns `Ok(())` if all assertions pass, or an error if any assertion fails.
    async fn assert_email_moved_and_item_removed(
        pool: Pool<MySql>,
        item: &TodoPimItem,
        full_paths: &HashMap<i64, String>,
        expected_timestamp: i64,
    ) -> Result<()> {
        // Get the mail directory path for the item's collection ID from the full paths mapping
        let mail_directory = full_paths
            .get(&item.collection_id)
            .ok_or_else(|| anyhow::anyhow!("Collection ID not found in full paths"))?;
        // Verify that the email file has been moved to the correct target maildir
        let pattern = format!("{}/cur/{}*S", mail_directory, expected_timestamp);
        let matching_file = get_single_matching_file(&pattern).await?;
        println!("Matching file found at: {}", matching_file);
        // Verify that the item.id has been cleared from the database.
        assert!(
            !assert_item_still_present_in_db(pool, item).await?,
            "Expected item with id {} to be removed from database",
            item.id
        );
        Ok(())
    }

    /// Helper function to test whether a specific item is still present in the database after processing.
    ///
    /// # Arguments
    /// - `pool`: The database connection pool to use for querying the database.
    /// - `item`: The `TodoPimItem` that was processed, containing the item ID and collection ID.
    ///
    /// # Returns
    /// - `Result<(bool)>`:
    ///   - `Ok(true)` if the item is still present in the database, indicating that it was not removed.
    ///  - `Ok(false)` if the item has been removed from the database, indicating that it was processed and deleted.
    ///  - An error if the database query fails, which could indicate an issue with the database connection or query execution.
    ///
    async fn assert_item_still_present_in_db(
        pool: Pool<MySql>,
        item: &TodoPimItem,
    ) -> Result<bool> {
        let row: (i32,) = sqlx::query_as("SELECT count(*) FROM pimitemtable WHERE id = ?")
            .bind(item.id)
            .fetch_one(&pool)
            .await?;
        Ok(row.0 > 0)
    }

    /// Test case to verify that an email with a odd remote ID is correctly moved to the target maildir
    /// and that the corresponding item is removed from the database.
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_moving_email_with_odd_name(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct without --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Take a test email item known to be in the test data
        let remote_id = "1291727681.2020.4jNSG:2,S".to_string();
        let item = TodoPimItem {
            id: 206,
            remote_id: Some(remote_id.clone()),
            collection_id: 388,
        };
        let expected_timestamp = 1686315625; // Extracted from email content

        let result = process_single_todo_item(pool.clone(), &item, &full_paths, &args).await;

        assert!(
            result.is_ok(),
            "Processing single todo item failed: {:?}",
            result
        );

        assert_email_moved_and_item_removed(pool.clone(), &item, &full_paths, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    /// Test case to verify that an email is not moved and the item is not removed from the database
    /// when the email is already in the right place.
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_email_already_in_right_place(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct without --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Take a test email item known to be in the test data
        let remote_id = "1330783242.R2038.sirius:2,S".to_string();
        let item = TodoPimItem {
            id: 1322,
            remote_id: Some(remote_id.clone()),
            collection_id: 66,
        };
        let expected_timestamp = 1330783242; // Extracted from email name

        let result = process_single_todo_item(pool.clone(), &item, &full_paths, &args).await;

        assert!(
            result.is_ok(),
            "Processing single todo item failed: {:?}",
            result
        );

        // Verify that the email file has NOT been moved (still exists in the same place)
        let pattern = format!(
            "{}/cur/{}*S",
            full_paths.get(&item.collection_id).unwrap(),
            expected_timestamp
        );
        let matching_file = get_single_matching_file(&pattern).await?;
        println!("Matching file found at: {}", matching_file);

        // Verify that the item.id has NOT been cleared from the database.
        assert!(
            assert_item_still_present_in_db(pool.clone(), &item).await?,
            "Expected item with id {} to still be present in database",
            item.id
        );

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    /// Test case to verify that an email is not moved and the item is not removed from the database
    /// when the email file is missing.
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_email_file_missing(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct without --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Take a test email item known to be in the database but with a missing email file
        let item = TodoPimItem {
            id: 132632,
            remote_id: None,
            collection_id: 42,
        };

        let result = process_single_todo_item(pool.clone(), &item, &full_paths, &args).await;

        assert!(
            result.is_ok(),
            "Processing single todo item failed: {:?}",
            result
        );

        // Verify that the item.id has been cleared from the database.
        assert!(
            !assert_item_still_present_in_db(pool.clone(), &item).await?,
            "Expected item with id {} to be cleared from database",
            item.id
        );

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    // Test case to verify that an email is not moved and the item is not removed from the database
    // when the ---dry-run flag is set.
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_dry_run_flag(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, true);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Take a test email item known to be in the test data
        let remote_id = "1491255228.R505.helios:2,PS".to_string();
        let item = TodoPimItem {
            id: 1207,
            remote_id: Some(remote_id.clone()),
            collection_id: 66,
        };

        let result = process_single_todo_item(pool.clone(), &item, &full_paths, &args).await;

        assert!(
            result.is_ok(),
            "Processing single todo item failed: {:?}",
            result
        );

        // Verify that the email file has NOT been moved (still exists in the same place)
        let pattern = format!(
            "{}/new/{}",
            full_paths.get(&item.collection_id).unwrap(),
            remote_id
        );
        let matching_file = get_single_matching_file(&pattern).await?;
        println!("Matching file found at: {}", matching_file);

        // Verify that the item.id has NOT been cleared from the database.
        assert!(
            assert_item_still_present_in_db(pool.clone(), &item).await?,
            "Expected item with id {} to still be present in database",
            item.id
        );

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    /// Test case to verify that an email is moved from "new" to "cur" and the item is removed from the database
    /// when the email is in the "new" directory and has a valid remote ID.
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_email_in_new_directory(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct without --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Take a test email item known to be in the "new" directory in the test data
        let remote_id = "1491255228.R505.helios:2,PS".to_string();
        let item = TodoPimItem {
            id: 1207,
            remote_id: Some(remote_id.clone()),
            collection_id: 66,
        };
        let expected_timestamp = 1491255228; // Extracted from email name

        let result = process_single_todo_item(pool.clone(), &item, &full_paths, &args).await;
        assert!(
            result.is_ok(),
            "Processing single todo item failed: {:?}",
            result
        );

        assert_email_moved_and_item_removed(pool.clone(), &item, &full_paths, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    /// Test case to verify that an email is moved and the item is removed from the database
    /// when the email is stored in the file_db cache directory (without a valid remote ID)
    /// and has a valid timestamp in the email content.
    ///
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_email_in_file_db_cache(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct without --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Take a test email item known to be in the file_db cache directory in the test data
        let item = TodoPimItem {
            id: 50642,
            remote_id: None,
            collection_id: 394,
        };
        let expected_timestamp = 1767092602; // Extracted from email content

        let result = process_single_todo_item(pool.clone(), &item, &full_paths, &args).await;
        assert!(
            result.is_ok(),
            "Processing single todo item failed: {:?}",
            result
        );

        assert_email_moved_and_item_removed(pool.clone(), &item, &full_paths, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    /// Test case to verify that an email is moved and the item is removed from the database
    /// when the email is stored in the database (without a valid remote ID)
    /// and has a valid timestamp in the email content.
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_email_in_database(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct without --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch full paths of all mail directories
        let full_paths: std::collections::HashMap<i64, String> =
            fetch_full_paths(pool.clone(), &args).await?;

        // Take a test email item known to be in the database in the test data
        let item = TodoPimItem {
            id: 50645,
            remote_id: None,
            collection_id: 394,
        };
        let expected_timestamp = 1767111447; // Extracted from email content

        let result = process_single_todo_item(pool.clone(), &item, &full_paths, &args).await;
        assert!(
            result.is_ok(),
            "Processing single todo item failed: {:?}",
            result
        );

        assert_email_moved_and_item_removed(pool.clone(), &item, &full_paths, expected_timestamp)
            .await?;

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }
}
