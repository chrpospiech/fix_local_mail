use crate::connect::{connect_to_database, get_database_url};

pub(crate) mod cmdline;
pub(crate) mod connect;
pub(crate) mod execute;
pub(crate) mod todoitems;

#[tokio::main]
async fn main() {
    let args = cmdline::parse_args();

    let dry_run = args.dry_run;
    if dry_run {
        println!("Dry run mode enabled. No changes will be made.");
    }
    // Connect to the database
    let database_url: String = get_database_url();
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&database_url).await;

    let todo_items: Vec<todoitems::TodoItem> = todoitems::fetch_todo_items(pool.clone()).await;

    // Print fetched data
    for item in todo_items {
        if dry_run {
            println!(
                "Dry run: would move {} to {}",
                item.source_path, item.target_path
            );
            // Remove temporary files created for cached emails
            execute::remove_temp_file(&item.source_path);
        } else {
            if args.verbose {
                println!(
                    "Processing item ID {}: moving {} to {}",
                    item.id, item.source_path, item.target_path
                );
            }
            // Move files to their target locations
            // We don't need to remove temp files separately here since they are moved
            execute::move_file(&item.source_path, &item.target_path);
            execute::update_akonadi_db(pool.clone(), &item.target_path, item.id).await;
        }
    }

    // Explicit disconnect
    pool.close().await;
}
