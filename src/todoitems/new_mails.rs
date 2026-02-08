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
use crate::process::maildirs::get_root_paths;
use anyhow::Result;
use regex::Regex;
use sqlx::{MySql, Pool};

pub async fn find_new_mail_files(pool: Pool<MySql>, args: &CliArgs) -> Result<Vec<String>> {
    let re = Regex::new(r"/new/(\d+.*\:2\,.*)$").unwrap();
    let mut matches = Vec::new();

    if args.ignore_new_dirs {
        if args.verbose || args.dry_run {
            println!("Ignoring new directories as per command line argument.");
        }
        return Ok(vec![]);
    }

    if args.verbose || args.dry_run {
        println!("Finding new mail files...");
    }

    let directories = get_root_paths(pool.clone(), args).await?;
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

    Ok(matches)
}
