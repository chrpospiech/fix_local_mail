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

pub async fn fetch_collections(
    pool: Pool<MySql>,
    get_root_only: bool,
) -> std::collections::HashMap<i64, Collection> {
    let mut query = sqlx::QueryBuilder::new(
        "
        SELECT `id`,
               CONVERT(`remoteId`, CHAR)    AS `remote_id`,
               CONVERT(`remoteRevision`, CHAR) AS `remote_revision`,
               CONVERT(`name`, CHAR)      AS `dir_name`,
               `parentId`    AS `parent_id`
        FROM `collectiontable`
        WHERE `resourceId` = 3
        ",
    );

    if get_root_only {
        query.push(" AND `parentId` IS NULL");
    }

    query
        .build_query_as::<Collection>()
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch collections")
        .into_iter()
        .map(|c| (c.id, c))
        .collect()
}

pub async fn get_root_paths(pool: Pool<MySql>) -> Vec<Option<String>> {
    let root_dirs: std::collections::HashMap<i64, Collection> =
        fetch_collections(pool.clone(), true).await;
    root_dirs
        .values()
        .map(|collection| collection.remote_id.clone())
        .collect::<Vec<Option<String>>>()
}
