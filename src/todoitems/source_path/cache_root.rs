#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{cmdline::CliArgs, todoitems::source_path::get_cache_root_path};

    #[test]
    fn test_get_cache_root_path_with_auto() {
        let args = CliArgs {
            mail_cache_path: "auto".to_string(),
            ..Default::default()
        };

        let result = get_cache_root_path(&args);

        // Should return the standard akonadi cache path
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        let akonadi_cache: String = format!("{}/.local/share/akonadi/file_db_data/", home_dir);
        assert!(!result.is_empty());
        assert!(result.ends_with('/'));
        assert_eq!(result, akonadi_cache);
    }

    #[test]
    fn test_get_cache_root_path_with_custom_path() {
        let custom_path = "/tmp/custom_cache";
        let args = CliArgs {
            mail_cache_path: custom_path.to_string(),
            ..Default::default()
        };

        let result = get_cache_root_path(&args);

        // Should return the custom path with trailing slash
        assert!(!result.is_empty());
        assert!(result.ends_with('/'));

        assert_eq!(result, PathBuf::from(custom_path));
    }
}
