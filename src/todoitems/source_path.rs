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

use crate::todoitems::CliArgs;
use anyhow::Result;
use sqlx::{MySql, Pool};
use std::io::Write;
use uuid::Uuid;

pub(crate) mod cache_root;
pub(crate) mod email_sources;
pub(crate) mod trashed_email;

pub async fn get_source_file_name(
    path: String,
    remote_id: Option<&String>,
    file_id: i64,
    pool: Pool<MySql>,
    args: &CliArgs,
) -> Result<Option<String>> {
    if let Some(rid) = remote_id {
        let pattern = format!("{}*/{}", path, rid);
        Ok(Some(get_single_matching_file(&pattern, args).await?))
    } else {
        get_cached_email(file_id, pool, args).await
    }
}

pub async fn get_single_matching_file(pattern: &str, args: &CliArgs) -> Result<String> {
    if args.db_url != "auto" {
        return Ok(pattern.to_string());
    }
    let mut paths = Vec::new();

    for entry in glob::glob(pattern)? {
        paths.push(entry?);
    }

    if paths.len() != 1 {
        anyhow::bail!("Expected exactly one file, found {}", paths.len());
    }

    Ok(paths[0].to_string_lossy().to_string())
}

pub fn get_cache_root_path(args: &CliArgs) -> Result<String> {
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
        Ok(cache_root_dir)
    } else {
        let home_dir = std::env::var("HOME")?;
        Ok(format!("{}/.local/share/akonadi/file_db_data/", home_dir))
    }
}

pub async fn get_cached_email(
    file_id: i64,
    pool: Pool<MySql>,
    args: &CliArgs,
) -> Result<Option<String>> {
    #[derive(sqlx::FromRow)]
    struct CachedEmail {
        data: Option<Vec<u8>>,
        storage: i32,
    }

    // Fetch cached email data from the database
    let result = sqlx::query_as::<_, CachedEmail>(
        "SELECT `data`, `storage` FROM `parttable` WHERE `pimItemId` = ? AND `partTypeId` = 2",
    )
    .bind(file_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        anyhow::anyhow!(
            "Failed to fetch cached email with given file_id {}: {}",
            file_id,
            e
        )
    })?;

    // Check if data is present
    if result.data.is_none() {
        if args.verbose || args.dry_run {
            println!(
                "Ignoring: No cached email found in database for file_id {}.",
                file_id
            );
        }
        return Ok(None);
    }
    let data = result.data.unwrap();

    // Convert data bytes to string
    let data_string = data.iter().map(|&b| b as char).collect::<String>();
    let cache_root_dir = get_cache_root_path(args)?;
    if result.storage == 1 {
        // Cached email is stored in file system
        let pattern = format!("{}*/{}", cache_root_dir, data_string);
        return Ok(get_single_matching_file(&pattern, args).await?.into());
    } else {
        // Cached email is stored in database
        let unique_name = format!("{}tmp{}", &cache_root_dir, Uuid::new_v4());
        if args.db_url == "auto" && !args.dry_run {
            let mut file = std::fs::File::create(&unique_name)?;
            file.write_all(&data)?;
        }
        Ok(Some(unique_name))
    }
}
