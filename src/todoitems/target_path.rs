use chrono::DateTime;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::mysql::MySql;
use sqlx::Pool;

use rand::Rng;
use regex::Regex;

pub async fn get_target_file_name(
    path: String,
    remote_id: Option<&String>,
    source_path: String,
    item_id: i64,
    pool: Pool<MySql>,
) -> String {
    let mail_name: String;
    if let Some(rid) = remote_id {
        // Extract mail name without flag info from remote_id using regex
        let re = Regex::new(r"(\d+\.R\d+\.\w+)").unwrap();
        if let Some(caps) = re.captures(rid) {
            mail_name = caps.get(1).unwrap().as_str().to_string();
        } else {
            panic!("Failed to extract mail name from remote_id: {}", rid);
        }
    } else {
        // Generate mail name based on timestamp, R value, and hostname
        let mail_time_stamp = get_mail_time_stamp(&source_path);
        let hostname = if mail_time_stamp < 1431532000 {
            // Before May 13, 2015, use "sirius" as hostname
            "sirius".to_string()
        } else {
            // After that, use actual hostname
            gethostname::gethostname()
                .into_string()
                .unwrap_or("unknownhost".to_string())
        };
        // Get R value from database
        let r_value = get_r_value(pool.clone(), mail_time_stamp).await;
        // Construct mail name
        mail_name = format!("{}.R{}.{}", mail_time_stamp, r_value, hostname);
    }
    // Get mail info (flags) from database
    let mail_info = get_mail_info(item_id, pool).await;
    // Construct final target file name with path, cur/new prefix, mail name, and mail info
    let cur_new_name = if mail_info.is_empty() { "new" } else { "cur" };
    format!("{}{}/{}{}", path, cur_new_name, mail_name, mail_info)
}

pub fn get_mail_time_stamp(mail_file: &str) -> u64 {
    // Open the mail file and read line by line to find the Date header
    let error_msg = format!("Cannot read mail file: {}", mail_file);
    let file = File::open(mail_file).expect(&error_msg);
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        // Look for the line starting with "Date: "
        if let Some(date_str) = line.strip_prefix("Date: ") {
            // Parse the date string using RFC 2822 format
            if let Ok(date_time) = DateTime::parse_from_rfc2822(date_str) {
                // Return the timestamp as seconds since UNIX_EPOCH
                return date_time.timestamp() as u64;
            }
        }
    }

    // If no date found or parsing failed, return current time in seconds since UNIX_EPOCH
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub async fn get_r_value(pool: Pool<MySql>, time_stamp: u64) -> u64 {
    // Find the maximum R value for mails with similar timestamp prefix
    let query = format!(
        "SELECT MAX(CONVERT(SUBSTR(REGEXP_SUBSTR(`remoteId`,'R[0-9]+'),2),UNSIGNED)) AS `r_value` \
             FROM `pimitemtable` \
             WHERE `mimeTypeId` = 2 \
             AND `remoteId` LIKE '{}%' \
             AND `collectionId` IN (SELECT id FROM `collectiontable` WHERE `resourceId` = 3)",
        time_stamp
    );

    // Execute the query
    let result: Option<(Option<u64>,)> = sqlx::query_as(&query)
        .fetch_optional(&pool)
        .await
        .expect("Failed to execute query");

    if let Some((Some(r_value),)) = result {
        // If an R value exists, increment it by a random number between 1 and 50
        // to avoid collisions with existing mails
        r_value + rand::rng().random_range(1..=50)
    } else {
        // If no R value exists, start with a random number between 20 and 950
        rand::rng().random_range(20..=950)
    }
}

pub async fn get_mail_info(file_id: i64, pool: Pool<MySql>) -> String {
    // Fetch mail flags from the database and construct the mail info string
    let query = format!(
        "SELECT `Flag_id` FROM `pimitemflagrelation` WHERE `PimItem_id` = {}",
        file_id
    );
    // Map of flag IDs to their corresponding characters
    let flag_map: HashMap<i32, &str> = HashMap::from([
        (1, "S"),
        (6, "P"),
        (9, "R"),
        (12, "R"),
        (14, "F"),
        (16, "T"),
    ]);

    // Execute the query to get flag IDs
    let rows: Vec<(i32,)> = sqlx::query_as(&query)
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch mail flags");

    // Construct the mail info string based on the fetched flags
    if rows.is_empty() {
        //
        String::new()
    } else {
        //
        let mut flags = rows
            .into_iter()
            .filter_map(|(flag_id,)| flag_map.get(&flag_id).copied())
            .collect::<Vec<&str>>();
        flags.sort();
        flags.dedup();
        let flag_string = flags.join("");
        // Return the mail info string in the format ":2,FLAGS"
        ":2,".to_string() + &flag_string
    }
}
