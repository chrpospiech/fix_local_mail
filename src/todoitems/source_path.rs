use crate::todoitems::CliArgs;
use sqlx::{MySql, Pool};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub(crate) mod cache_root;

pub async fn get_source_file_name(
    path: String,
    remote_id: Option<&String>,
    file_id: i64,
    pool: Pool<MySql>,
    args: &CliArgs,
) -> String {
    if args.db_url != "socket" {
        if let Some(rid) = remote_id {
            return format!("tbd/{}", rid);
        } else {
            return "tbd/NULL".to_string();
        }
    }
    if let Some(rid) = remote_id {
        let pattern = format!("{}*/{}", path, rid);
        get_single_matching_file(&pattern).await
    } else {
        get_cached_email(file_id, pool, args).await
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

pub fn get_cache_root_path(args: &CliArgs) -> String {
    if args.mail_cache_path != "auto" {
        let cache_root_dir = if args.mail_cache_path.ends_with('/') {
            args.mail_cache_path.clone()
        } else {
            format!("{}/", args.mail_cache_path)
        };
        if args.verbose || args.dry_run {
            println!(
                "Using source root path from command line argument: {}",
                cache_root_dir
            );
        }
        cache_root_dir
    } else {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        format!("{}/.local/share/akonadi/file_db_data/", home_dir)
    }
}

pub async fn get_cached_email(file_id: i64, pool: Pool<MySql>, args: &CliArgs) -> String {
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
    let cache_root_dir = get_cache_root_path(args);
    if result.storage == 1 {
        // Cached email is stored in file system
        let pattern = format!("{}*/{}", cache_root_dir, data_string);
        return get_single_matching_file(&pattern).await;
    } else {
        // Cached email is stored in database
        let unique_name = format!("{}/tmp{}", cache_root_dir, Uuid::new_v4());
        let path = PathBuf::from(&unique_name);
        let mut file = std::fs::File::create(&path).expect("Failed to create temp file");
        file.write_all(&result.data)
            .expect("Failed to write to temp file");
        unique_name
    }
}
