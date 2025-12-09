use regex::Regex;
use std::path::PathBuf;
use tokio::fs;

pub async fn find_new_mail_files(directories: Vec<Option<String>>) -> Vec<String> {
    let re = Regex::new(r"/new/(\d+\.R\d+.*)$").unwrap();
    let mut matches = Vec::new();

    for dir in directories.into_iter().flatten() {
        let mut stack = vec![PathBuf::from(dir)];

        while let Some(current_dir) = stack.pop() {
            let mut entries = fs::read_dir(&current_dir)
                .await
                .expect("Failed to read directory");

            while let Some(entry) = entries
                .next_entry()
                .await
                .expect("Failed to read directory entry")
            {
                let path = entry.path();

                if path.is_dir() {
                    stack.push(path);
                } else if let Some(path_str) = path.to_str() {
                    if let Some(caps) = re.captures(path_str) {
                        if let Some(matched) = caps.get(1) {
                            matches.push(matched.as_str().to_string());
                        }
                    }
                }
            }
        }
    }

    matches
}
