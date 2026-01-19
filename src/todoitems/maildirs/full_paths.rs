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
mod tests {

    use std::collections::HashMap;

    use crate::{cmdline::CliArgs, todoitems::maildirs::fetch_full_paths};
    use sqlx::MySqlPool;

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_full_paths_from_args(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Setup an argument struct
        let args = CliArgs {
            maildir_path: "/tmp/maildir/path".to_string(),
            ..Default::default()
        };
        // Test: Retrieve the root path
        let result: HashMap<i64, String> = fetch_full_paths(pool.clone(), &args).await;
        for (_key, value) in result.iter() {
            assert!(value.starts_with(&args.maildir_path));
        }

        Ok(())
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_full_paths_default(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Setup an argument struct
        let args = CliArgs {
            maildir_path: "auto".to_string(),
            ..Default::default()
        };
        // Test: Retrieve the root path
        let result: HashMap<i64, String> = fetch_full_paths(pool.clone(), &args).await;
        for (_key, value) in result.iter() {
            assert!(value.starts_with("/home/cp/.local/share/akonadi_maildir_resource_0/"));
        }

        Ok(())
    }
}
