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
    use std::path::PathBuf;

    use anyhow::Result;

    use crate::{cmdline::CliArgs, todoitems::source_path::get_cache_root_path};

    #[test]
    fn test_get_cache_root_path_with_auto() -> Result<()> {
        let args = CliArgs {
            mail_cache_path: "auto".to_string(),
            ..Default::default()
        };

        let result = get_cache_root_path(&args);

        // Should return the standard akonadi cache path
        let home_dir = std::env::var("HOME")?;
        let akonadi_cache: String = format!("{}/.local/share/akonadi/file_db_data/", home_dir);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_empty());
        assert!(result.ends_with('/'));
        assert_eq!(result, akonadi_cache);
        Ok(())
    }

    #[test]
    fn test_get_cache_root_path_with_custom_path() {
        let custom_path = "/tmp/custom_cache";
        let args = CliArgs {
            mail_cache_path: custom_path.to_string(),
            ..Default::default()
        };

        let result = get_cache_root_path(&args);

        assert!(result.is_ok());
        let result = result.unwrap();

        // Should return the custom path with trailing slash
        assert!(!result.is_empty());
        assert!(result.ends_with('/'));

        assert_eq!(result, PathBuf::from(custom_path));
    }
}
