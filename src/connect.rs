use sqlx::mysql::MySqlPool;

pub async fn connect_to_database(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(database_url).await
}
