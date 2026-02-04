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
/// - `test_moving_stored_email`: Verifies that a stored email is moved to the correct
///   target maildir and that the Akonadi database is updated accordingly.
mod tests {
    use crate::mockup::{create_test_cli_args, setup_tmp_mail_dir, teardown_tmp_mail_dir};
    use crate::process::{
        maildirs::fetch_full_paths, process_single_todo_item, source_path::get_single_matching_file,
    };
    use crate::todoitems::TodoPimItem;
    use anyhow::{Ok, Result};
    use sqlx::{MySql, Pool};

    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_moving_stored_email(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, true);

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

        // Verify that the email file has been moved to the correct target maildir
        let mail_directory = full_paths
            .get(&item.collection_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Collection ID {} not found in full paths mapping.",
                    item.collection_id
                )
            })?
            .clone();
        let pattern = format!("{}/cur/{}*S", mail_directory, expected_timestamp);
        let matching_file = get_single_matching_file(&pattern, &args).await?;
        println!("Matching file found at: {}", matching_file);

        // Verify that the item.id has been cleared from the database.
        let row: (i32,) = sqlx::query_as("SELECT count(*) FROM pimitemtable WHERE id = ?")
            .bind(item.id)
            .fetch_one(&pool)
            .await?;
        assert_eq!(
            row.0, 0,
            "Expected item with id {} to be removed from database",
            item.id
        );

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }
}
