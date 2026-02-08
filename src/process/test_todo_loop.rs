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
mod test {
    use crate::mockup::{create_test_cli_args, setup_tmp_mail_dir, teardown_tmp_mail_dir};
    use crate::process::process_todo_items;
    use crate::todoitems::{fetch_todo_pim_items, TodoPimItem};
    use anyhow::Result;
    use sqlx::{MySql, Pool};
    use walkdir::WalkDir;

    /// Test the process_todo_items function by setting up a temporary mail directory,
    /// creating test CLI arguments, and asserting that the function executes without errors.
    /// This test uses the akonadi.sql fixture to populate the database with test data.
    /// The following assertions are made:
    /// - The process_todo_items function returns Ok(()).
    /// - A second call to fetch_todo_pim_items returns an empty vector,
    ///   indicating that all items were processed.
    /// - The temporary mail directory contains no files after processing
    ///   in any of the ".inbox.directory/*/new" subdirectories of temp_dir.
    /// - The database has only one row in the pimitemtable left after processing.
    ///
    #[sqlx::test(fixtures("../../tests/fixtures/akonadi.sql"))]
    async fn test_process_todo_items(pool: Pool<MySql>) -> Result<()> {
        // Recursively copy src/todoitems/tests/data to a unique subdirectory in /tmp
        let temp_dir: String = setup_tmp_mail_dir()?;

        // Setup an argument struct without --dry-run, pointing to the temporary mail directory
        let args = create_test_cli_args(&temp_dir, false);

        // Call the function to test
        let result = process_todo_items(pool.clone(), &args).await;

        // Assert that the function returns Ok(())
        assert!(result.is_ok());

        // Assert that a second call to fetch_todo_pim_items returns an empty vector
        let remaining_items: Vec<TodoPimItem> = fetch_todo_pim_items(pool.clone(), &args).await?;
        assert!(remaining_items.is_empty());

        // Assert that the temporary mail directory contains no files after processing
        // in any of the ".inbox.directory/*/new" subdirectories of temp_dir
        let inbox_dir = std::path::Path::new(&temp_dir).join("local_mail").join(".inbox.directory");
        let mut file_count = 0;
        if inbox_dir.exists() {
            // Walk the directory tree:
            // - min_depth(1) skips .inbox.directory itself
            // - max_depth(2) reaches the "new" directories under each mailbox subfolder
            //   (depth 1 = mailbox folders like "nirwana", depth 2 = "new" directories)
            for entry in WalkDir::new(&inbox_dir)
                .min_depth(1)
                .max_depth(2)
                .into_iter()
                .filter_entry(|e| e.file_type().is_dir())
            {
                let entry = entry?; // Propagate any filesystem errors
                if entry.file_name() == std::ffi::OsStr::new("new") {
                    file_count += std::fs::read_dir(entry.path())?.count();
                }
            }
        }
        assert_eq!(
            file_count, 0,
            "Expected no files in the new directories after processing"
        );

        // Assert that the database has only one row in the pimitemtable left after processing
        let row_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pimitemtable")
            .fetch_one(&pool)
            .await?;
        assert_eq!(
            row_count.0, 1,
            "Expected exactly one row left in the pimitemtable after processing"
        );

        // Cleanup the temporary mail directory
        teardown_tmp_mail_dir(&temp_dir)?;

        Ok(())
    }
}
