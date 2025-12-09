use crate::connect::{connect_to_database, get_fallback_url};
use crate::maildirs::fetch_collections;
use crate::new_mails::find_new_mail_files;
use crate::todoitems::fetch_todo_items;

pub(crate) mod connect;
pub(crate) mod maildirs;
pub(crate) mod new_mails;
pub(crate) mod todoitems;

#[tokio::main]
async fn main() {
    // Connect to the database
    let current_url: String = std::env::var("DATABASE_URL").unwrap_or_else(|_| get_fallback_url());
    let pool: sqlx::Pool<sqlx::MySql> = connect_to_database(&current_url).await;

    // Fetch mail root directories
    let root_paths: Vec<Option<String>> = maildirs::get_root_paths(pool.clone()).await;
    // Find new mail files
    let new_mail_list: Vec<String> = find_new_mail_files(root_paths).await;
    let collections: std::collections::HashMap<i64, maildirs::Collection> =
        fetch_collections(pool.clone(), false).await;
    let todo_items: Vec<todoitems::TodoItem> = fetch_todo_items(pool.clone(), new_mail_list).await;

    // Print fetched data
    for item in todo_items {
        println!("{:?}", item);
        println!("{:?}", collections.get(&item.collection_id));
    }

    println!("Hello, world!");

    // Explicit disconnect
    pool.close().await;
}
