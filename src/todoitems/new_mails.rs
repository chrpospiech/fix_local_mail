use regex::Regex;

pub async fn find_new_mail_files(directories: Vec<Option<String>>) -> Vec<String> {
    let re = Regex::new(r"/new/(\d+.*\:2\,.*)$").unwrap();
    let mut matches = Vec::new();

    for dir in directories.into_iter().flatten() {
        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(path_str) = entry.path().to_str() {
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
