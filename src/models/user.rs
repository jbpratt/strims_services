#![allow(dead_code)]
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::schema::users;
use crate::server::DbPool;

#[derive(Queryable, Debug, Clone, Insertable)]
pub struct User {
    pub id: String,
    pub twitch_id: i64,
    pub name: String,
    pub stream_path: String,
    pub service: String,
    pub channel: String,
    pub last_ip: String,
    pub last_seen: NaiveDateTime,
    pub left_chat: Option<bool>,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_admin: Option<bool>,
}

pub fn get_user_by_id(uid: Uuid, pool: &DbPool) -> Result<Option<User>, ApiError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<User>(&conn)
        .optional()?;

    Ok(user)
}

pub fn get_user_by_twitch_id(tid: i64, pool: &DbPool) -> Result<Option<User>, ApiError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;

    let user = users
        .filter(twitch_id.eq(tid))
        .first::<User>(&conn)
        .optional()?;

    Ok(user)
}

pub fn get_user_by_name(user_name: &str, pool: &DbPool) -> Result<Option<User>, ApiError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;

    let user = users
        .filter(name.eq(user_name))
        .first::<User>(&conn)
        .optional()?;

    Ok(user)
}

pub fn get_user_by_stream_path(
    user_stream_path: &str,
    pool: &DbPool,
) -> Result<Option<User>, ApiError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;

    let user = users
        .filter(stream_path.eq(user_stream_path))
        .first::<User>(&conn)
        .optional()?;

    Ok(user)
}

/*
pub fn create_user(
    twitch_id: u64,
    chn: Channel,
    ip: &str,
    pool: &DbPool,
) -> Result<Option<User>, ApiError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;

    let new_user = User {
        twitch_id,
    };

    diesel::insert_into(users).values(new_user).execute(&conn)?;
    Ok(new_user.clone().into())
}
*/
