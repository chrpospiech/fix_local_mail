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
use crate::todoitems::TodoPimItem;
use anyhow::Result;
use chrono::DateTime;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::mysql::MySql;
use sqlx::Pool;

use rand::Rng;
use regex::Regex;

pub(crate) mod email_targets;

pub async fn get_target_file_name(
    pool: Pool<MySql>,
    item: &TodoPimItem,
    full_paths: &HashMap<i64, String>,
    time_stamp: u64,
) -> Result<String> {
    let mail_name: String;
    let re = Regex::new(r"(\d+\.R\d+\.\w+)").unwrap();
    if let Some(rid) = item.remote_id.as_ref() {
        if let Some(caps) = re.captures(rid) {
            // Use existing mail name from remote ID
            mail_name = caps.get(1).unwrap().as_str().to_string();
        } else {
            // Generate mail name based on timestamp, R value, and hostname
            mail_name = create_new_mail_name(pool.clone(), time_stamp).await?;
        }
    } else {
        // Generate mail name based on timestamp, R value, and hostname
        mail_name = create_new_mail_name(pool.clone(), time_stamp).await?;
    }
    // Get mail info (flags) from database
    let mail_info = get_mail_info(item.id, pool).await?;
    // Construct final target file name with path, cur/new prefix, mail name, and mail info
    let cur_new_name = if mail_info.is_empty() { "new" } else { "cur" };
    let directory_path = full_paths
        .get(&item.collection_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Collection ID {} not found in full paths mapping.",
                item.collection_id
            )
        })?
        .clone();
    let path = if directory_path.ends_with('/') {
        directory_path
    } else {
        format!("{}/", directory_path)
    };
    Ok(format!(
        "{}{}/{}{}",
        path, cur_new_name, mail_name, mail_info
    ))
}

pub async fn create_new_mail_name(pool: Pool<MySql>, time_stamp: u64) -> Result<String> {
    // Generate mail name based on timestamp, R value, and hostname
    let mail_time_stamp = time_stamp;
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap_or("unknownhost".to_string());
    // Get R value from database
    let r_value = get_r_value(pool.clone(), mail_time_stamp).await?;
    // Construct mail name
    Ok(format!("{}.R{}.{}", mail_time_stamp, r_value, hostname))
}

pub fn get_mail_time_stamp(mail_file: &str, args: &CliArgs) -> Result<u64> {
    if args.db_url != "auto" || args.dry_run {
        if args.verbose || args.dry_run {
            println!(
                "Custom DB URL or dry run: Not looking for mail timestamp from file {}.",
                mail_file
            );
        }

        /*
        If args.db_url != "auto", return current time in seconds since UNIX_EPOCH minus
        a random value between 1 and 1800 to avoid collisions - simulating mails received in the recent past
        */
        let random_offset: u64 = rand::rng().random_range(1..=1800);
        return Ok(get_time_now_secs()?.saturating_sub(random_offset));
    }
    // Open the mail file and read line by line to find the Date header
    let file = File::open(mail_file)
        .map_err(|e| anyhow::anyhow!("Cannot read mail file: {}: {}", mail_file, e))?;
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        // Look for the line starting with "Date: "
        if let Some(date_str) = line.strip_prefix("Date: ") {
            // Parse the date string using RFC 2822 format
            if let Ok(date_time) = DateTime::parse_from_rfc2822(date_str) {
                // Return the timestamp as seconds since UNIX_EPOCH
                return Ok(date_time.timestamp() as u64);
            }
        }
    }

    /*
    If no date found or parsing failed, return current time in seconds since UNIX_EPOCH minus
    a random value between 1 and 1800 to avoid collisions - simulating mails received in the recent past
    */
    let random_offset: u64 = rand::rng().random_range(1..=1800);
    Ok(get_time_now_secs()?.saturating_sub(random_offset))
}

pub fn get_time_now_secs() -> Result<u64> {
    // Get the current system time in seconds since UNIX_EPOCH
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| anyhow::anyhow!("Time went backwards"))?
        .as_secs())
}

pub async fn get_r_value(pool: Pool<MySql>, time_stamp: u64) -> Result<u64> {
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
    let result: Option<(Option<u64>,)> = sqlx::query_as(&query).fetch_optional(&pool).await?;

    if let Some((Some(r_value),)) = result {
        // If an R value exists, increment it by a random number between 1 and 50
        // to avoid collisions with existing mails
        Ok(r_value + rand::rng().random_range(1..=50))
    } else {
        // If no R value exists, start with a random number between 20 and 950
        Ok(rand::rng().random_range(20..=950))
    }
}

pub async fn get_mail_info(file_id: i64, pool: Pool<MySql>) -> Result<String> {
    // Fetch mail flags from the database and construct the mail info string
    // the `flagtable`.`id` entries, might be different for each user.
    // Hence they should not be used in SQL queries. Instead a four letter
    // acronym `flag` is used instead.
    let query = format!(
        "SELECT SUBSTR(CONVERT(`flagtable`.`name`, CHAR), 2,4) AS `flag`
         FROM `pimitemflagrelation` RIGHT JOIN `flagtable`
           ON `pimitemflagrelation`.`Flag_id` = `flagtable`.`id`
        WHERE `pimitemflagrelation`.`PimItem_id` = {}",
        file_id
    );
    // Map of flags to their corresponding characters
    let flag_map: HashMap<&str, &str> = HashMap::from([
        ("SEEN", "S"),
        ("FORW", "P"),
        ("ANSW", "R"),
        ("REPL", "R"),
        ("FLAG", "F"),
        ("DELE", "T"),
    ]);

    // Execute the query to get flags
    let rows: Vec<(String,)> = sqlx::query_as(&query).fetch_all(&pool).await?;

    // Construct the mail info string based on the fetched flags
    if rows.is_empty() {
        //
        Ok(String::new())
    } else {
        //
        let mut flags = rows
            .into_iter()
            .filter_map(|(flag,)| flag_map.get(flag.as_str()).copied())
            .collect::<Vec<&str>>();
        flags.sort();
        flags.dedup();
        let flag_string = flags.join("");
        if !flag_string.is_empty() {
            // Prepend :2, if there are flags
            Ok(":2,".to_string() + &flag_string)
        } else {
            // No flags found - return empty string
            Ok(String::new())
        }
    }
}
