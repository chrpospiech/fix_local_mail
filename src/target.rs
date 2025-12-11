use chrono::DateTime;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_mail_time_stamp(mail_file: &str) -> u64 {
    let error_msg = format!("Cannot read mail file: {}", mail_file);
    let file = File::open(mail_file).expect(&error_msg);
    let reader = BufReader::new(file);

    for line in reader.lines().map_while(Result::ok) {
        if let Some(date_str) = line.strip_prefix("Date: ") {
            if let Ok(date_time) = DateTime::parse_from_rfc2822(date_str) {
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
