#[cfg(test)]
mod tests {

    use crate::{cmdline::CliArgs, todoitems::maildirs::get_root_paths};
    use sqlx::MySqlPool;

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_root_path_from_args(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Setup an argument struct
        let args = CliArgs {
            maildir_path: "/tmp/maildir/path".to_string(),
            ..Default::default()
        };
        // Test: Retrieve the root path
        let result: Vec<Option<String>> = get_root_paths(pool.clone(), &args).await;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Some("/tmp/maildir/path".to_string()));

        Ok(())
    }

    #[sqlx::test(fixtures("../tests/fixtures/akonadi.sql"))]
    pub async fn test_get_root_path_default(
        pool: MySqlPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Setup an argument struct
        let args = CliArgs {
            maildir_path: "auto".to_string(),
            ..Default::default()
        };
        // Test: Retrieve the root path
        let result: Vec<Option<String>> = get_root_paths(pool.clone(), &args).await;
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            Some("/home/cp/.local/share/akonadi_maildir_resource_0/".to_string())
        );

        Ok(())
    }
}
