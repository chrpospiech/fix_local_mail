pub(crate) mod connect;
pub(crate) mod maildirs;

#[tokio::main]
async fn main() {
    // Connect to the database
    let default_url = "mysql://lmxtest:lmxtest@localhost/akonadi";
    let pool = connect::connect_to_database(default_url).await;

    let collections = maildirs::fetch_collections(pool.clone()).await;
    for collection in collections {
        println!("{:?}", collection);
    }

    println!("Hello, world!");

    // Explicit disconnect
    pool.close().await;
}
