use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::channel::{get_channel_id, Channel};
use crate::database::DbPool;
use crate::errors::ApiError;
use crate::schema::streams;

#[derive(Debug, Queryable, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
pub struct Stream {
    pub id: Option<i64>,
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

fn insert(pool: &DbPool, mut stream: Stream) -> anyhow::Result<Stream, ApiError> {
    use crate::schema::streams::dsl::streams;

    let conn = pool.get()?;

    let channel = Channel::new(
        stream.channel.clone(),
        stream.service.clone(),
        stream.path.clone().unwrap_or(String::new()),
    )?;

    let id = get_channel_id(&channel);
    stream.id = Some(id as i64);

    diesel::insert_into(streams)
        .values(stream.clone())
        .execute(&conn)?;
    Ok(stream)
}

fn update(pool: &DbPool, mut stream: Stream) -> anyhow::Result<(), ApiError> {
    use crate::schema::streams::dsl::{id, streams};

    let conn = pool.get()?;

    if stream.id.is_none() {
        let channel = Channel::new(
            stream.channel.clone(),
            stream.service.clone(),
            stream.path.clone().unwrap_or(String::new()),
        )?;

        stream.id = Some(get_channel_id(&channel) as i64);
    }

    diesel::update(streams)
        .filter(id.eq(stream.id.clone()))
        .set(stream)
        .execute(&conn)?;
    Ok(())
}

fn get_by_id(pool: &DbPool, stream_id: i64) -> anyhow::Result<Stream, ApiError> {
    use crate::schema::streams::dsl::{id, streams};

    let conn = pool.get()?;

    let stream = streams
        .filter(id.eq(stream_id))
        .first::<Stream>(&conn)
        .map_err(|_| ApiError::NotFound(format!("Stream not found with id: {:?}", stream_id)))?;

    Ok(stream)
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

        let result = insert(&pool, stream.clone());
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.id.is_some());
        assert_eq!(result.service, stream.service);
        assert_eq!(result.viewers, stream.viewers);
    }

    #[test]
    fn it_inserts_finds_and_updates_a_stream() {
        let pool = setup_pool();
        let stream = Stream {
            service: String::from("twitch"),
            channel: String::from("jbpratt"),
            viewers: Some(5),
            live: Some(true),
            ..Default::default()
        };

        let result = insert(&pool, stream.clone());
        assert!(result.is_ok());

        let stream = result.unwrap();

        let inserted_stream = get_by_id(&pool, stream.id.unwrap().clone());
        assert!(inserted_stream.is_ok());

        let mut stream = inserted_stream.unwrap();
        stream.viewers = Some(50);

        let result = update(&pool, stream.clone());
        assert!(result.is_ok());

        let updated_stream = get_by_id(&pool, stream.id.unwrap().clone());
        assert!(updated_stream.is_ok());

        assert_eq!(updated_stream.unwrap().viewers.unwrap(), 50);
    }

    #[test]
    fn it_doesnt_find_a_stream() {
        let pool = setup_pool();
        let stream = get_by_id(&pool, 1);
        assert!(stream.is_err());
    }
}
