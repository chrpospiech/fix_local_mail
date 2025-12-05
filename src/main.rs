pub(crate) mod connect;
pub(crate) mod maildirs;
pub(crate) mod todoitems;

#[tokio::main]
async fn main() {
    // Connect to the database
    let default_url = "mysql://lmxtest:lmxtest@localhost/akonadi";
    let pool = connect::connect_to_database(default_url).await;

    let collections = maildirs::fetch_collections(pool.clone()).await;
    let todo_items = todoitems::fetch_todo_items(pool.clone()).await;

    for item in todo_items {
        println!("{:?}", item);
        println!("{:?}", collections.get(&item.collection_id));
    }

    println!("Hello, world!");

    // Explicit disconnect
    pool.close().await;
}
