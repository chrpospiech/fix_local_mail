use crate::connect::{connect_to_database, get_database_url};

pub(crate) mod connect;
pub(crate) mod todoitems;

#[tokio::main]
async fn main() {
    // Connect to the database
    let database_url: String = get_database_url();
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&database_url).await;

    let todo_items: Vec<todoitems::TodoItem> = todoitems::fetch_todo_items(pool.clone()).await;

    // Print fetched data
    for item in todo_items {
        println!("{:?}", item);
        // Remove temporary files created for cached emails
        if item.source_path.starts_with("/tmp/") {
            if let Err(e) = std::fs::remove_file(&item.source_path) {
                eprintln!("Failed to remove temp file {}: {}", item.source_path, e);
            }
        }
    }

    // Explicit disconnect
    pool.close().await;
}
