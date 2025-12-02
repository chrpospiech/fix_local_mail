pub(crate) mod connect;

#[tokio::main]
async fn main() {
    // Connect to the database
    let default_url = "mysql://lmxtest:lmxtest@localhost/akonadi";
    let pool = match connect::connect_to_database(default_url).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };
    println!("Hello, world!");

    // Explicit disconnect
    pool.close().await;
}
