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
