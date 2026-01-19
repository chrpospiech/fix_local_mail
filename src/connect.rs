// Copyright 2026 fix_local_mail C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cmdline::CliArgs;
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
        panic!("Failed to get MySQL auto path. Is the Akonadi database running?");
    }
    let sock = socket.unwrap();
    format!("mysql://localhost/akonadi?socket={}", sock)
}

pub async fn connect_to_database(args: &CliArgs) -> sqlx::Pool<sqlx::MySql> {
    let database_url: String = if args.db_url == "auto" {
        get_database_url()
    } else {
        args.db_url.clone()
    };
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    pool
}
