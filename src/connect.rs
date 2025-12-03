use sqlx::mysql::MySqlPool;

pub async fn connect_to_database(database_url: &str) -> sqlx::Pool<sqlx::MySql> {
    let pool = MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to database");
    pool
}
