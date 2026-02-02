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
    use anyhow::Result;

    use crate::{
        cmdline::CliArgs, connect::connect_to_database, todoitems::source_path::get_cached_email,
    };

    #[tokio::test]
    async fn test_get_cached_email() -> Result<()> {
        let args = CliArgs {
            mail_cache_path: "/home/cp/tools/Rust/snapshot_gesche/mail_cache/".to_string(),
            db_url: "mysql://cp:IdNut,olif6kneurfibs@localhost/akonadi".to_string(),
            maildir_path: "/home/cp/tools/Rust/snapshot_gesche/maildir/".to_string(),
            dry_run: true,
            ..Default::default()
        };

        let pool = connect_to_database(&args).await?;

        // Assuming file_id 1 exists in the test database
        let file_id = 132632;
        let result = get_cached_email(file_id, pool, &args).await;

        // Check that the result is not empty
        assert!(!result.is_empty());

        Ok(())
    }
}
