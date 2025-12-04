use sqlx::{FromRow, MySql, Pool};

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct Collection {
    pub id: i64,
    pub remote_id: Option<String>,
    pub remote_revision: Option<String>,
    pub dir_name: String,
    pub parent_id: Option<i64>,
}

pub async fn fetch_collections(pool: Pool<MySql>) -> std::collections::HashMap<i64, Collection> {
    sqlx::query_as::<_, Collection>(
        "
        SELECT `id`,
               CONVERT(`remoteId`, CHAR)    AS `remote_id`,
               CONVERT(`remoteRevision`, CHAR) AS `remote_revision`,
               CONVERT(`name`, CHAR)      AS `dir_name`,
               `parentId`    AS `parent_id`
        FROM `collectiontable`
        WHERE `resourceId` = 3
        ",
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch collections")
    .into_iter()
    .map(|c| (c.id, c))
    .collect()
}
