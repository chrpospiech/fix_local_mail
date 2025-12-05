use crate::connect::{connect_to_database, get_fallback_url};

pub(crate) mod connect;
pub(crate) mod maildirs;
pub(crate) mod todoitems;

#[tokio::main]
async fn main() {
    // Connect to the database
    let current_url: String = std::env::var("DATABASE_URL").unwrap_or_else(|_| get_fallback_url());
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&current_url).await;

    // Fetch and print mail collections and todo items
    let collections: std::collections::HashMap<i64, maildirs::Collection> =
        maildirs::fetch_collections(pool.clone()).await;
    let mail_list: Vec<String> = vec!["1759397542456.R839.helios:2,S".to_string()];
    let todo_items: Vec<todoitems::TodoItem> =
        todoitems::fetch_todo_items(pool.clone(), mail_list).await;

    // Print fetched data
    for item in todo_items {
        println!("{:?}", item);
        println!("{:?}", collections.get(&item.collection_id));
    }

    println!("Hello, world!");

    // Explicit disconnect
    pool.close().await;
}
