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
use sqlx::{FromRow, MySql, Pool};
use std::collections::HashMap;

pub(crate) mod full_paths;
pub(crate) mod root_paths;

#[derive(Debug, Clone, FromRow)]
#[allow(dead_code)]
pub struct Collection {
    pub id: i64,
    pub remote_id: Option<String>,
    pub remote_revision: Option<String>,
    pub dir_name: String,
    pub parent_id: Option<i64>,
}

pub async fn fetch_collections(
    pool: Pool<MySql>,
    get_root_only: bool,
) -> Result<HashMap<i64, Collection>> {
    let mut query = sqlx::QueryBuilder::new(
        "
        SELECT `id`,
               CONVERT(`remoteId`, CHAR)    AS `remote_id`,
               CONVERT(`remoteRevision`, CHAR) AS `remote_revision`,
               CONVERT(`name`, CHAR)      AS `dir_name`,
               `parentId`    AS `parent_id`
        FROM `collectiontable`
        WHERE `resourceId` = 3
        ",
    );

    if get_root_only {
        query.push(" AND `parentId` IS NULL");
    }

    let result = query
        .build_query_as::<Collection>()
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|c| (c.id, c))
        .collect();
    Ok(result)
}

pub async fn get_root_paths(pool: Pool<MySql>, args: &CliArgs) -> Result<Vec<Option<String>>> {
    if args.maildir_path != "auto" {
        if args.verbose || args.dry_run {
            println!(
                "Using maildir root path from command line argument: {}",
                args.maildir_path
            );
        }
        return Ok(vec![Some(args.maildir_path.clone())]);
    }
    let root_dirs: std::collections::HashMap<i64, Collection> =
        fetch_collections(pool.clone(), true).await?;
    Ok(root_dirs
        .values()
        .map(|collection| collection.remote_id.clone())
        .collect::<Vec<Option<String>>>())
}

// Recursively build the full path for a collection by traversing its parent collections.
// The resulting paths are stored in the `paths` HashMap.
// This auxiliary function is used to help construct full paths for collections.
#[allow(dead_code)]
pub fn set_parent_paths(
    id: i64,
    collections: HashMap<i64, Collection>,
    paths: &mut HashMap<i64, String>,
    args: &CliArgs,
) -> Result<()> {
    let collection = collections
        .get(&id)
        .ok_or_else(|| anyhow::anyhow!("Collection not found: {}", id))?;
    if paths.contains_key(&id) {
        return Ok(());
    }
    if collection.parent_id.is_none() {
        let root_path = if args.maildir_path != "auto" {
            &args.maildir_path
        } else {
            collection
                .remote_id
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Root collection has NULL remote_id"))?
        };
        let path = if root_path.ends_with('/') {
            root_path.clone()
        } else {
            format!("{}/", root_path)
        };
        paths.insert(id, path);
    } else {
        let parent_id = collection
            .parent_id
            .ok_or_else(|| anyhow::anyhow!("Parent ID should be Some"))?;
        set_parent_paths(parent_id, collections.clone(), paths, args)?;
        let parent_path = paths
            .get(&parent_id)
            .ok_or_else(|| anyhow::anyhow!("Parent path not found"))?;
        let remote_id = collection
            .remote_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Collection has NULL remote_id"))?;
        let path = format!("{}.{}.directory/", parent_path, remote_id);
        paths.insert(id, path);
    }
    Ok(())
}

pub async fn fetch_full_paths(pool: Pool<MySql>, args: &CliArgs) -> Result<HashMap<i64, String>> {
    let collections: HashMap<i64, Collection> = fetch_collections(pool.clone(), false).await?;
    let mut paths: HashMap<i64, String> = HashMap::new();

    for id in collections.keys() {
        set_parent_paths(*id, collections.clone(), &mut paths, args)?;
    }

    let result = collections
        .keys()
        .map(|id| -> anyhow::Result<(i64, String)> {
            let collection = collections
                .get(id)
                .ok_or_else(|| anyhow::anyhow!("Collection not found"))?;
            let path = paths
                .get(id)
                .ok_or_else(|| anyhow::anyhow!("Path not found"))?;
            let full_path = if collection.parent_id.is_none() {
                path.clone()
            } else {
                let parent_path = paths
                    .get(
                        &collection
                            .parent_id
                            .ok_or_else(|| anyhow::anyhow!("Parent ID should be Some"))?,
                    )
                    .ok_or_else(|| anyhow::anyhow!("Parent path not found"))?;
                let remote_id = collection
                    .remote_id
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("Collection has NULL remote_id"))?;
                format!("{}{}/", parent_path, remote_id)
            };
            Ok((*id, full_path))
        })
        .collect::<anyhow::Result<HashMap<i64, String>>>()?;
    Ok(result)
}
