use crate::connect::{connect_to_database, get_fallback_url};
use crate::new_mails::find_new_mail_files;
use crate::todoitems::fetch_todo_items;

pub(crate) mod connect;
pub(crate) mod maildirs;
pub(crate) mod new_mails;
pub(crate) mod target;
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
    // Fetch full paths of all mail directories
    let full_paths: std::collections::HashMap<i64, String> =
        maildirs::fetch_full_paths(pool.clone()).await;
    // Fetch todo items corresponding to new mail files
    let todo_items: Vec<todoitems::TodoItem> = fetch_todo_items(pool.clone(), new_mail_list).await;

    // Print fetched data
    for item in todo_items {
        println!("{:?}", full_paths.get(&item.collection_id));
        println!("{:?}", item);
    }

    let test_mail_file = "/home/cp/Mail/AltHendesse/cur/1540826869.R19.helios:2,S";
    println!(
        "Mail {} has timestamp: {}",
        test_mail_file,
        target::get_mail_time_stamp(test_mail_file)
    );

    // Explicit disconnect
    pool.close().await;
}
