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

use crate::cmdline::CliArgs;
use anyhow::Result;

/// Sets up a temporary mail directory structure for testing purposes.
///
/// This function creates a temporary directory with a unique UUID-based name and
/// recursively copies the test data from `src/todoitems/tests/data` into it.
///
/// # Returns
///
/// Returns the path to the created temporary directory as a `String`.
///
/// # Errors
///
/// Returns an `anyhow::Error` if any file system operation fails.
///
pub fn setup_tmp_mail_dir() -> Result<String> {
    // Create a temporary mail directory structure for testing
    // Recursively copy src/todoitems/tests/data to this structure
    let temp_dir = std::env::temp_dir().join(format!("maildir_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir)?;
    let manifest_dir = env!("CARGO_MANIFEST_DIR"); // compile-time
    let path = std::path::Path::new(manifest_dir).join("tests/data");
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;
    fs_extra::dir::copy(&path, &temp_dir, &options)?;

    Ok(temp_dir.to_string_lossy().to_string())
}

/// Cleans up a temporary mail directory created by `setup_tmp_mail_dir`.
///
/// # Arguments
///
/// * `temp_dir` - Path to the temporary directory to remove
///
/// # Errors
///
/// Returns an `anyhow::Error` if the directory removal fails.
///
pub fn teardown_tmp_mail_dir(temp_dir: &str) -> Result<()> {
    std::fs::remove_dir_all(temp_dir)?;
    Ok(())
}

/// Creates a `CliArgs` instance configured for testing.
///
/// # Arguments
///
/// * `temp_dir` - Base path for the temporary test directory
/// * `dry_run` - dry run flag to set in the arguments
///
/// # Returns
///
/// Returns a `CliArgs` instance with maildir and cache paths configured to use
/// subdirectories within the provided `temp_dir`.
///
pub fn create_test_cli_args(temp_dir: &str, dry_run: bool) -> CliArgs {
    CliArgs {
        maildir_path: format!("{}/local_mail/", temp_dir),
        mail_cache_path: format!("{}/file_db_data/", temp_dir),
        dry_run,
        ..Default::default()
    }
}
