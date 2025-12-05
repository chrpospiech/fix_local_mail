use sqlx::QueryBuilder;
use sqlx::{FromRow, MySql, Pool};

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct TodoItem {
    pub id: i64,
    pub remote_id: Option<String>,
    pub collection_id: i64,
    pub dirty: bool,
    pub mime_type_id: i64,
}

pub async fn fetch_todo_items(pool: Pool<MySql>, mail_list: Vec<String>) -> Vec<TodoItem> {
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
