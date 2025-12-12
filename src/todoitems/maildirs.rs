use sqlx::{FromRow, MySql, Pool};

#[derive(Debug, Clone, FromRow)]
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

// Recursively build the full path for a collection by traversing its parent collections.
// The resulting paths are stored in the `paths` HashMap.
// This auxiliary function is used to help construct full paths for collections.
#[allow(dead_code)]
pub fn set_parent_paths(
    id: i64,
    collections: std::collections::HashMap<i64, Collection>,
    paths: &mut std::collections::HashMap<i64, String>,
) {
    let collection = collections.get(&id).expect("Collection not found");
    if paths.contains_key(&id) {
        return;
    }
    if collection.parent_id.is_none() {
        let remote_id = collection
            .remote_id
            .as_ref()
            .expect("Root collection has NULL remote_id");
        let path = if remote_id.ends_with('/') {
            remote_id.clone()
        } else {
            format!("{}/", remote_id)
        };
        paths.insert(id, path);
    } else {
        let parent_id = collection.parent_id.expect("Parent ID should be Some");
        set_parent_paths(parent_id, collections.clone(), paths);
        let parent_path = paths.get(&parent_id).expect("Parent path not found");
        let remote_id = collection
            .remote_id
            .as_ref()
            .expect("Collection has NULL remote_id");
        let path = format!("{}.{}.directory/", parent_path, remote_id);
        paths.insert(id, path);
    }
}

pub async fn fetch_full_paths(pool: Pool<MySql>) -> std::collections::HashMap<i64, String> {
    let collections: std::collections::HashMap<i64, Collection> =
        fetch_collections(pool.clone(), false).await;
    let mut paths: std::collections::HashMap<i64, String> = std::collections::HashMap::new();

    for id in collections.keys() {
        set_parent_paths(*id, collections.clone(), &mut paths);
    }

    collections
        .keys()
        .map(|id| {
            let collection = collections.get(id).expect("Collection not found");
            let path = paths.get(id).expect("Path not found");
            let full_path = if collection.parent_id.is_none() {
                path.clone()
            } else {
                let parent_path = paths
                    .get(&collection.parent_id.expect("Parent ID should be Some"))
                    .expect("Parent path not found");
                let remote_id = collection
                    .remote_id
                    .as_ref()
                    .expect("Collection has NULL remote_id");
                format!("{}{}/", parent_path, remote_id)
            };
            (*id, full_path)
        })
        .collect()
}
