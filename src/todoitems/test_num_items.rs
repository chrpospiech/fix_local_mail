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
/// This module contains integration tests that verify generation of the to-do list.
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
/// - `test_default_number_of_todoitems`:
///   Verifies that all todo items are fetched when no filters are applied
/// - `test_number_of_todoitems_ignoring_new`:
///   Verifies that todo items from 'new' directories are excluded when `ignore_new_dirs` is enabled
/// - `test_number_of_todoitems_below_limit`:
///   Verifies that the number of fetched todo items respects the specified limit
/// - `test_number_of_todoitems_above_minimum_id`:
///   Verifies that only todo items with IDs greater than or equal to `min_id` are fetched
///
mod tests {

    use crate::cmdline::CliArgs;
    use crate::mockup::{create_test_cli_args, setup_tmp_mail_dir, teardown_tmp_mail_dir};
    use crate::todoitems::fetch_todo_pim_items;
    use anyhow::Result;
    use sqlx::mysql::MySqlPool;

    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_default_number_of_todoitems(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = create_test_cli_args(&temp_dir, false);

        // Fetch todo items from the database
        let todo_items = fetch_todo_pim_items(pool.clone(), &args).await?;

        // Verify the number of todo items fetched
        assert_eq!(todo_items.len(), 9);

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_number_of_todoitems_ignoring_new(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = CliArgs {
            maildir_path: format!("{}/local_mail/", temp_dir),
            mail_cache_path: format!("{}/file_db_data/", temp_dir),
            db_url: "auto".to_string(),
            ignore_new_dirs: true,
            ..Default::default()
        };

        // Fetch todo items from the database ignoring new directories
        let todo_items = fetch_todo_pim_items(pool.clone(), &args).await?;

        // Verify the number of todo items fetched
        assert_eq!(todo_items.len(), 6);

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_number_of_todoitems_below_limit(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = CliArgs {
            maildir_path: format!("{}/local_mail/", temp_dir),
            mail_cache_path: format!("{}/file_db_data/", temp_dir),
            db_url: "auto".to_string(),
            limit: 5,
            ..Default::default()
        };

        // Fetch todo items from the database
        let todo_items = fetch_todo_pim_items(pool.clone(), &args).await?;

        // Verify the number of todo items is below the given limit
        assert_eq!(todo_items.len(), 5);

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }

    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    pub async fn test_number_of_todoitems_above_minimum_id(pool: MySqlPool) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct with db_url = "auto"
        let args = CliArgs {
            maildir_path: format!("{}/local_mail/", temp_dir),
            mail_cache_path: format!("{}/file_db_data/", temp_dir),
            db_url: "auto".to_string(),
            min_id: 50642,
            ..Default::default()
        };

        // Fetch todo items from the database ignoring new directories
        let todo_items = fetch_todo_pim_items(pool.clone(), &args).await?;

        // Verify the number of todo items fetched
        assert_eq!(todo_items.len(), 4);

        // Clean up: Remove the temporary directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }
}
