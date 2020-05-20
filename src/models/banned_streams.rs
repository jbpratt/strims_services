use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::channel::valid_service;
use crate::database::DbPool;
use crate::errors::ApiError;
use crate::schema::banned_streams;

#[derive(Queryable, Debug, Identifiable, Clone, Insertable, Deserialize, Serialize)]
#[primary_key(channel, service)]
pub struct BannedStream {
    pub channel: String,
    pub service: String,
    pub reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for BannedStream {
    fn default() -> Self {
        Self {
            channel: String::new(),
            service: String::new(),
            reason: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

fn get_all(pool: &DbPool) -> anyhow::Result<Vec<BannedStream>, ApiError> {
    let conn = pool.get()?;
    let all = banned_streams::table.load(&conn)?;
    Ok(all)
}

fn insert(pool: &DbPool, stream: &BannedStream) -> anyhow::Result<BannedStream, ApiError> {
    let conn = pool.get()?;

    if !valid_service(stream.service.as_str()) {
        return Err(ApiError::InvalidService(stream.service.clone()));
    }

    diesel::insert_into(banned_streams::table)
        .values(stream)
        .execute(&conn)?;
    Ok(stream.clone())
}

fn remove(pool: &DbPool, stream: &BannedStream) -> anyhow::Result<(), ApiError> {
    use crate::schema::banned_streams::dsl::{channel, service};

    let conn = pool.get()?;
    diesel::delete(
        banned_streams::table
            .filter(channel.eq(stream.channel.clone()))
            .filter(service.eq(stream.service.clone())),
    )
    .execute(&conn)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::setup_pool;

    #[test]
    fn it_inserts_a_banned_stream() {
        let pool = setup_pool();
        let banned_stream = BannedStream {
            channel: String::from("jbpratt"),
            service: String::from("twitch"),
            reason: Some(String::from("because")),
            ..Default::default()
        };

        let created = insert(&pool, &banned_stream.clone());
        assert!(created.is_ok());

        let stream = created.unwrap();
        assert_eq!(banned_stream.channel, stream.channel);
    }

    #[test]
    fn it_inserts_and_gets_all_banned_streams() {
        let pool = setup_pool();
        let banned_stream_1 = BannedStream {
            channel: String::from("jbpratt"),
            service: String::from("twitch"),
            reason: Some(String::from("because")),
            ..Default::default()
        };
        let banned_stream_2 = BannedStream {
            channel: String::from("jbpratt"),
            service: String::from("angelthump"),
            reason: Some(String::from("because")),
            ..Default::default()
        };

        let created = insert(&pool, &banned_stream_1.clone());
        assert!(created.is_ok());
        let created = insert(&pool, &banned_stream_2.clone());
        assert!(created.is_ok());

        let all = get_all(&pool);
        assert!(all.is_ok());

        let all = all.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn it_inserts_and_removes_a_banned_stream() {
        let pool = setup_pool();
        let banned_stream = BannedStream {
            channel: String::from("jbpratt"),
            service: String::from("twitch"),
            reason: Some(String::from("because")),
            ..Default::default()
        };

        let created = insert(&pool, &banned_stream.clone());
        assert!(created.is_ok());

        let removed = remove(&pool, &banned_stream.clone());
        assert!(removed.is_ok());
    }

    #[test]
    fn it_fails_to_insert_due_to_invalid_service() {
        let pool = setup_pool();
        let banned_stream = BannedStream {
            channel: String::from("jbpratt"),
            service: String::from("twitter"),
            reason: Some(String::from("because")),
            ..Default::default()
        };

        let created = insert(&pool, &banned_stream.clone());
        assert!(created.is_err());

        let expected_err = ApiError::InvalidService(String::from("twitter"));
        assert_eq!(created.unwrap_err(), expected_err);
    }
}
