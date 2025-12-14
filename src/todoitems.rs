use futures::future::join_all;
use sqlx::QueryBuilder;
use sqlx::{FromRow, MySql, Pool};

pub(crate) mod maildirs;
pub(crate) mod new_mails;
pub(crate) mod source_path;
pub(crate) mod target_path;

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct TodoPimItem {
    pub id: i64,
    pub remote_id: Option<String>,
    pub collection_id: i64,
    pub dirty: bool,
    pub mime_type_id: i64,
}

pub async fn fetch_todo_pim_items(pool: Pool<MySql>, mail_list: Vec<String>) -> Vec<TodoPimItem> {
    let mut query_builder = QueryBuilder::new(
        "SELECT `id`,
            CONVERT(`remoteId`, CHAR) AS `remote_id`,
            `collectionId` AS `collection_id`,
            `dirty`,
            `mimeTypeId` AS `mime_type_id`
        FROM `pimitemtable`
        WHERE `mimeTypeId` = 2
        AND (`dirty` = 1 OR `remoteId` NOT LIKE '%:2%S'",
    );

    if !mail_list.is_empty() {
        query_builder.push(" OR `remoteId` IN (");
        let mut separated = query_builder.separated(", ");
        for mail in &mail_list {
            separated.push_bind(mail);
        }
        query_builder.push(")");
    }

    query_builder.push(
        ")
        AND `collectionId` IN (
            SELECT id FROM `collectiontable` WHERE `resourceId` = 3
        )",
    );

    query_builder
        .build_query_as::<_>()
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch mail todo items")
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TodoItem {
    pub id: i64,
    pub dirty: bool,
    pub source_path: String,
    pub target_path: String,
}

pub async fn fetch_todo_items(pool: Pool<MySql>) -> Vec<TodoItem> {
    // Fetch mail root directories
    let root_paths: Vec<Option<String>> = maildirs::get_root_paths(pool.clone()).await;
    // Find new mail files
    let new_mail_list: Vec<String> = new_mails::find_new_mail_files(root_paths).await;
    // Fetch full paths of all mail directories
    let full_paths: std::collections::HashMap<i64, String> =
        maildirs::fetch_full_paths(pool.clone()).await;
    // Fetch todo items corresponding to new mail files
    let todo_pim_items: Vec<TodoPimItem> = fetch_todo_pim_items(pool.clone(), new_mail_list).await;

    let async_todo_items = todo_pim_items
        .into_iter()
        .map(|item| {
            let pool = pool.clone();
            let full_path = full_paths
                .get(&item.collection_id)
                .cloned()
                .unwrap_or("tbd/".to_string());
            async move {
                let item_source = source_path::get_source_file_name(
                    full_path.clone(),
                    item.remote_id.as_ref(),
                    item.id,
                    pool.clone(),
                )
                .await;
                let item_target = target_path::get_target_file_name(
                    full_path,
                    item.remote_id.as_ref(),
                    item_source.clone(),
                    item.id,
                    pool.clone(),
                )
                .await;
                TodoItem {
                    id: item.id,
                    dirty: item.dirty,
                    source_path: item_source,
                    target_path: item_target,
                }
            }
        })
        .collect::<Vec<_>>();

    let todo_items = join_all(async_todo_items).await;

    todo_items
}
