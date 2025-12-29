use crate::connect::connect_to_database;

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
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&args).await;

    let todo_items: Vec<todoitems::TodoItem> =
        todoitems::fetch_todo_items(pool.clone(), &args).await;

    // Handle fetched data
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
            execute::update_akonadi_db(pool.clone(), item.id).await;
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
        execute::clean_up(args.stop_akonadi, args.stop_kmail).await;
    }
}
