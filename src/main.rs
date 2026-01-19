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

use crate::connect::connect_to_database;
use anyhow::Result;

pub(crate) mod cmdline;
pub(crate) mod connect;
pub(crate) mod execute;
pub(crate) mod todoitems;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cmdline::parse_args();

    let dry_run = args.dry_run || &args.db_url != "auto";
    if dry_run {
        println!("Dry run mode enabled. No changes will be made.");
    }
    // Connect to the database
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&args).await?;

    let todo_items: Vec<todoitems::TodoItem> =
        todoitems::fetch_todo_items(pool.clone(), &args).await?;

    // Handle fetched data
    for item in todo_items {
        if dry_run {
            println!(
                "Dry run item ID {}: would move {} to {}",
                item.id, item.source_path, item.target_path
            );
        } else {
            // Move files to their target locations if source and target are not the same
            if item.source_path != item.target_path {
                if args.verbose {
                    println!(
                        "Processing item ID {}: moving {} to {}",
                        item.id, item.source_path, item.target_path
                    );
                }
                execute::move_file(&item.source_path, &item.target_path)?;
                execute::update_akonadi_db(pool.clone(), item.id).await?;
            } else if args.verbose {
                println!(
                    "Item ID {}: source and target path {} are the same. No action taken.",
                    item.id, item.source_path
                );
            }
        }
    }

    // Explicit disconnect from the database
    pool.close().await;

    // Clean up operations
    if dry_run {
        println!("Dry run: would clean up Akonadi and KMail.");
    } else {
        if args.verbose {
            println!("Cleaning up Akonadi and KMail...");
        }
        execute::clean_up(args.stop_akonadi, args.stop_kmail).await?;
    }
    Ok(())
}
