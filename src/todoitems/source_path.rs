use sqlx::{MySql, Pool};

pub async fn get_source_file_name(
    path: String,
    remote_id: Option<&String>,
    file_id: i64,
    pool: Pool<MySql>,
) -> String {
    if let Some(rid) = remote_id {
        let pattern = format!("{}*/{}", path, rid);
        get_single_matching_file(&pattern).await
    } else {
        get_cached_email(file_id, pool).await
    }
}

pub async fn get_single_matching_file(pattern: &str) -> String {
    let mut paths = Vec::new();

    for entry in glob::glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => panic!("Glob error: {}", e),
        }
    }

    if paths.len() != 1 {
        panic!("Expected exactly one file, found {}", paths.len());
    }

    paths[0].to_string_lossy().to_string()
}

pub async fn get_cached_email(file_id: i64, pool: Pool<MySql>) -> String {
    #[derive(sqlx::FromRow)]
    struct CachedEmail {
        data: Vec<u8>,
        storage: i32,
    }

    // Fetch cached email data from the database
    let result = sqlx::query_as::<_, CachedEmail>(
        "SELECT `data`, `storage` FROM `parttable` WHERE `pimItemId` = ? AND `partTypeId` = 2",
    )
    .bind(file_id)
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch cached email");

    // Convert data bytes to string
    let data_string = result.data.iter().map(|&b| b as char).collect::<String>();
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    if result.storage == 1 {
        // Cached email is stored in file system
        let pattern = format!(
            "{}/.local/share/akonadi/file_db_data/*/{}",
            home_dir, data_string
        );
        return get_single_matching_file(&pattern).await;
    } // Cached email is stored in database

    format!("stored {}", result.storage)
}
