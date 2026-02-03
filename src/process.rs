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
use crate::process::execute::{move_file, update_akonadi_db};
use crate::process::maildirs::fetch_full_paths;
use crate::process::source_path::get_source_file_name;
use crate::process::target_path::get_target_file_name;
use crate::todoitems::{fetch_todo_pim_items, TodoPimItem};
use anyhow::Result;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

pub(crate) mod execute;
pub(crate) mod maildirs;
pub(crate) mod source_path;
pub(crate) mod target_path;

pub async fn process_todo_items(pool: Pool<MySql>, args: &CliArgs) -> Result<()> {
    // Fetch mail directory tree with full paths
    let full_paths = fetch_full_paths(pool.clone(), args).await?;

    // get todo pim items
    let todo_items: Vec<TodoPimItem> = fetch_todo_pim_items(pool.clone(), args).await?;

    for item in todo_items {
        process_single_todo_item(pool.clone(), &item, &full_paths, args).await?;
    }

    Ok(())
}

async fn process_single_todo_item(
    pool: Pool<MySql>,
    item: &TodoPimItem,
    full_paths: &HashMap<i64, String>,
    args: &CliArgs,
) -> Result<()> {
    let dry_run_msg_start = if args.dry_run {
        "Dry run"
    } else {
        "Processing"
    };
    let dry_run_msg_would = if args.dry_run { "Would move" } else { "Moving" };

    if args.verbose || args.dry_run {
        println!(
            "Processing Todo Item ID: {}, Remote ID: {:?}, Collection ID: {}",
            item.id, item.remote_id, item.collection_id
        );
    }

    let dummy_path = "/dummy/path/for/testing".to_string();
    let source = get_source_file_name(pool.clone(), item, full_paths, args).await?;
    let target = get_target_file_name(
        dummy_path.clone(),
        item.remote_id.as_ref(),
        dummy_path.clone(),
        item.id,
        pool.clone(),
        args,
    )
    .await?;
    if source.is_none() {
        if args.verbose || args.dry_run {
            println!(
                "{} item ID {}: source path does not exist. Remove from database.",
                dry_run_msg_start, item.id
            );
        }
        if !args.dry_run {
            update_akonadi_db(pool.clone(), item.id).await?;
        }
        return Ok(());
    }
    let source = source.as_ref().unwrap();
    if source != &target {
        if args.verbose || args.dry_run {
            println!(
                "{} item ID {}: {} {} to {}",
                dry_run_msg_start, item.id, dry_run_msg_would, source, target
            );
        }
        if !args.dry_run {
            move_file(source, &target)?;
            update_akonadi_db(pool.clone(), item.id).await?;
        }
    } else if args.verbose || args.dry_run {
        println!(
            "{} item ID {}: source and target paths are the same. No action taken.",
            dry_run_msg_start, item.id
        );
    }
    Ok(())
}
