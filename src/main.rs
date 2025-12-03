pub(crate) mod connect;

#[tokio::main]
async fn main() {
    // Connect to the database
    let default_url = "mysql://lmxtest:lmxtest@localhost/akonadi";
    let pool = connect::connect_to_database(default_url).await;

    println!("Hello, world!");

    // Explicit disconnect
    pool.close().await;
}
