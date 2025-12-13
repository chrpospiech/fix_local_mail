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

pub fn get_database_url() -> String {
    let socket: Option<String> = get_mysql_socket();
    if socket.is_none() {
        panic!("Failed to get MySQL socket path. Is the Akonadi database running?");
    }
    let sock = socket.unwrap();
    format!("mysql://localhost/akonadi?socket={}", sock)
}

pub async fn connect_to_database(database_url: &str) -> sqlx::Pool<sqlx::MySql> {
    let pool = MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to database");
    pool
}
