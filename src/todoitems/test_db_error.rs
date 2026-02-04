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
/// Tests module for handling database errors when fetching todo pim items.
///
/// These tests ensure that the application behaves correctly
/// when the underlying database tables are missing or inaccessible.
///
/// # Test setup
///
/// 1. No temporary mail directory structure is created for testing.
/// 2. A SQLite database is initialized in a temporary location.
/// 3. The necessary tables are intentionally not created to simulate database errors.
///
mod tests {
    use crate::cmdline::CliArgs;
    use crate::todoitems::fetch_todo_pim_items;
    use anyhow::Result;
    use sqlx::mysql::MySqlPool;

    #[sqlx::test(fixtures("../../tests/fixtures/broken.sql"))]
    /// Tests that fetching todo pim items fails gracefully when the database tables are missing.
    async fn test_fetch_todo_pim_items_db_error(pool: MySqlPool) -> Result<()> {
        // We don't need to set up a temporary mail directory for this test
        // However, we suppress looking for new mails by setting ignore_new_dirs to true.
        // This focuses the test on database error handling.
        // Setup an argument struct
        let args = CliArgs {
            maildir_path: String::new(),
            mail_cache_path: String::new(),
            dry_run: true,
            db_url: "auto".to_string(),
            ignore_new_dirs: true,
            ..Default::default()
        };
        let result = fetch_todo_pim_items(pool.clone(), &args).await;
        assert!(
            result.is_err(),
            "Expected an error due to missing database tables."
        );

        Ok(())
    }
}
