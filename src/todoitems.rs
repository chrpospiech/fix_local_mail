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
use anyhow::Result;
use sqlx::QueryBuilder;
use sqlx::{FromRow, MySql, Pool};

pub(crate) mod new_mails;

#[derive(Debug, FromRow)]
pub struct TodoPimItem {
    pub id: i64,
    pub remote_id: Option<String>,
    pub collection_id: i64,
}

pub async fn fetch_todo_pim_items(pool: Pool<MySql>, args: &CliArgs) -> Result<Vec<TodoPimItem>> {
    // Build the query starting with mails that have `Id >= args.min_id`
    // and are marked as dirty or new
    let mut query_builder = QueryBuilder::new(
        "SELECT `id`,
            CONVERT(`remoteId`, CHAR) AS `remote_id`,
            `collectionId` AS `collection_id`
        FROM `pimitemtable`
        WHERE `mimeTypeId` = 2
        AND `id` >= ",
    );

    query_builder.push_bind(args.min_id);
    query_builder.push(" AND (`dirty` = 1 OR `remoteId` NOT LIKE '%:2,%S'");

    // Include items flagged as `\ANSWERED` but not marked as replied
    // These items also need to be processed and the flag
    // changed to replied after moving.
    query_builder.push(
        " OR (
        `id` IN (SELECT pimItem_Id
                 FROM `pimitemflagrelation`
                 WHERE `flag_Id` IN (SELECT `id`
                                     FROM `flagtable`
                                     WHERE `name` LIKE '%ANSWERED'))
        AND `remoteId` NOT LIKE '%:2%RS')",
    );

    // Add remote IDs from mail_list to the query
    let mail_list = new_mails::find_new_mail_files(pool.clone(), args).await?;
    if !mail_list.is_empty() {
        query_builder.push(" OR `remoteId` IN (");
        let mut separated = query_builder.separated(", ");
        for mail in &mail_list {
            separated.push_bind(mail);
        }
        query_builder.push(")");
    }

    // Close the main WHERE clause selecting only mails in local folders
    query_builder.push(
        ")
        AND `collectionId` IN (
            SELECT id FROM `collectiontable` WHERE `resourceId` = 3
        )",
    );

    // Add limit if specified in args
    if args.limit > 0 {
        query_builder.push(format!(
            "
        ORDER BY `id`
        LIMIT {}",
            args.limit
        ));
    }

    let query = query_builder.build_query_as::<_>().fetch_all(&pool).await?;
    Ok(query)
}
