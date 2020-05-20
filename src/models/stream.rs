use chrono::{NaiveDateTime, Utc};

use crate::database::DbPool;
use crate::errors::ApiError;
use crate::schema::streams;

#[derive(Debug, Queryable, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
pub struct Stream {
    pub id: Option<i32>,
    pub service: String,
    pub channel: String,
    pub path: Option<String>,
    pub hidden: Option<bool>,
    pub afk: Option<bool>,
    pub promoted: Option<bool>,
    pub title: String,
    pub thumbnail: Option<String>,
    pub live: Option<bool>,
    pub viewers: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for Stream {
    fn default() -> Self {
        Self {
            id: None,
            service: String::new(),
            channel: String::new(),
            path: None,
            hidden: None,
            afk: None,
            promoted: None,
            title: String::new(),
            thumbnail: None,
            live: None,
            viewers: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

fn insert(pool: &DbPool, stream: &Stream) -> anyhow::Result<Stream, ApiError> {
    //    get_by_id(pool, stream.id)
    todo!()
}

fn update(pool: &DbPool, stream: &Stream) -> anyhow::Result<Stream, ApiError> {
    todo!()
}

fn get_by_id(pool: &DbPool, id: i32) -> anyhow::Result<Stream, ApiError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::setup_pool;

    #[test]
    fn it_inserts_a_stream() {
        let pool = setup_pool();
        let stream = Stream {
            service: String::from("twitch"),
            channel: String::from("jbpratt"),
            viewers: Some(5),
            live: Some(true),
            ..Default::default()
        };

        let result = insert(&pool, &stream);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.service, stream.service);
        assert_eq!(result.viewers, stream.viewers);
    }

    fn it_doesnt_find_a_stream() {
        todo!()
    }
}
