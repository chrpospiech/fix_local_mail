use regex::Regex;
use sqlx::mysql::MySqlPool;
use std::process::Command;

pub fn get_mysql_socket() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ps aux | grep mysqld | grep socket")
        .output()
        .ok()?;

    let stdout = String::from_utf8(output.stdout).ok()?;
    let re = Regex::new(r"--socket=(\S+)\s").ok()?;

    re.captures(&stdout)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

pub fn get_fallback_url() -> String {
    let fallback_url: &str = "mysql://lmxtest:lmxtest@localhost/akonadi";
    let socket: Option<String> = get_mysql_socket();
    if let Some(sock) = socket {
        format!("mysql://lmxtest:lmxtest@localhost/akonadi?socket={}", sock)
    } else {
        fallback_url.to_string()
    }
}

pub async fn connect_to_database(database_url: &str) -> sqlx::Pool<sqlx::MySql> {
    let pool = MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to database");
    pool
}
