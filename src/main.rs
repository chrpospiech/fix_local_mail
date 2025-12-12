use crate::connect::{connect_to_database, get_fallback_url};

pub(crate) mod connect;
pub(crate) mod todoitems;

#[tokio::main]
async fn main() {
    // Connect to the database
    let current_url: String = std::env::var("DATABASE_URL").unwrap_or_else(|_| get_fallback_url());
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&current_url).await;

    let todo_items: Vec<todoitems::TodoItem> = todoitems::fetch_todo_items(pool.clone()).await;

    // Print fetched data
    for item in todo_items {
        println!("{:?}", item);
    }

    // Explicit disconnect
    pool.close().await;
}
