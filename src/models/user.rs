use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::channel::Channel;
use crate::database::DbPool;
use crate::errors::ApiError;
use crate::schema::users;

#[derive(Queryable, Debug, Clone, Insertable, PartialEq, AsChangeset)]
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

pub fn get_by_id(pool: &DbPool, uid: Uuid) -> anyhow::Result<User, ApiError> {
    use crate::schema::users::dsl::id;

    let conn = pool.get()?;
    Ok(users::table
        .filter(id.eq(uid.to_string()))
        .first::<User>(&conn)
        .map_err(|_| {
            ApiError::NotFound(format!("failed to find user with id: {}", uid.to_string()))
        })?)
}

pub fn get_by_twitch_id(pool: &DbPool, tid: i64) -> anyhow::Result<User, ApiError> {
    use crate::schema::users::dsl::twitch_id;

    let conn = pool.get()?;

    Ok(users::table
        .filter(twitch_id.eq(tid))
        .first::<User>(&conn)
        .map_err(|_| {
            ApiError::NotFound(format!(
                "failed to find user with twitch_id: {}",
                tid.to_string()
            ))
        })?)
}

pub fn get_by_name(pool: &DbPool, user_name: &str) -> anyhow::Result<User, ApiError> {
    use crate::schema::users::dsl::name;

    let conn = pool.get()?;

    Ok(users::table
        .filter(name.eq(user_name))
        .first::<User>(&conn)
        .map_err(|_| {
            ApiError::NotFound(format!(
                "failed to find user with name: {}",
                user_name.to_string()
            ))
        })?)
}

pub fn get_by_stream_path(pool: &DbPool, user_stream_path: &str) -> anyhow::Result<User, ApiError> {
    use crate::schema::users::dsl::stream_path;

    let conn = pool.get()?;

    Ok(users::table
        .filter(stream_path.eq(user_stream_path))
        .first::<User>(&conn)
        .map_err(|_| {
            ApiError::NotFound(format!(
                "failed to find user with stream_path: {}",
                user_stream_path.to_string()
            ))
        })?)
}

pub fn create(
    pool: &DbPool,
    twitch_id: i64,
    chn: Channel,
    name: &str,
    ip: &str,
) -> anyhow::Result<User, ApiError> {
    use crate::schema::users::dsl::users;
    let conn = pool.get()?;

    let stream_path = if chn.stream_path.is_empty() {
        format!("/{}/{}", chn.service, chn.channel)
    } else {
        chn.stream_path.clone()
    };

    let new_user = User {
        id: Uuid::new_v4().to_hyphenated().to_string(),
        twitch_id,
        name: name.to_string(),
        last_ip: ip.to_string(),
        left_chat: None,
        is_admin: None,
        channel: chn.channel,
        service: chn.service,
        stream_path,
        ..Default::default()
    };

    let out = new_user.clone();
    diesel::insert_into(users).values(new_user).execute(&conn)?;

    log::info!("inserted {:?} into the users table", out);
    Ok(out)
}

pub fn update(pool: &DbPool, update_user: &User) -> anyhow::Result<(), ApiError> {
    use crate::schema::users::dsl::{id, users};

    let conn = pool.get()?;

    diesel::update(users)
        .filter(id.eq(update_user.id.clone()))
        .set(update_user)
        .execute(&conn)?;

    /*
    get_user_by_id(
        &pool.clone(),
        Uuid::parse_str(&update_user.id).map_err(|e| ApiError::CannotParseUuid(e.to_string()))?,
    )
    */
    Ok(())
}

impl Default for User {
    fn default() -> Self {
        Self {
            twitch_id: 0,
            id: String::new(),
            name: String::new(),
            last_ip: String::new(),
            channel: String::new(),
            service: String::new(),
            stream_path: String::new(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            last_seen: Utc::now().naive_utc(),
            ban_reason: None,
            left_chat: None,
            is_admin: None,
            is_banned: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::setup_pool;

    fn create_test_user(pool: &DbPool) -> anyhow::Result<User, ApiError> {
        create(
            &pool,
            8,
            Channel {
                channel: String::from("jbpratt"),
                service: String::from("twitch"),
                stream_path: String::new(),
            },
            "jbpratt",
            "0.0.0.0",
        )
    }

    #[test]
    fn it_creates_a_user_and_finds_by_id() {
        let pool = setup_pool();
        let user = create_test_user(&pool);

        assert!(user.is_ok());
        let unwrapped = user.unwrap();

        let id = Uuid::parse_str(unwrapped.id.as_str()).unwrap();
        let found_user = get_by_id(&pool, id).unwrap();
        assert_eq!(unwrapped.name, found_user.name);
        assert_eq!(unwrapped.last_ip, found_user.last_ip);
        assert_eq!(unwrapped.service, found_user.service);
        assert_eq!(unwrapped.stream_path, found_user.stream_path);
    }

    #[test]
    fn it_doesnt_find_a_user() {
        let user_id = Uuid::new_v4();
        let not_found_user = get_by_id(&setup_pool(), user_id);
        assert!(not_found_user.is_err());
    }

    #[test]
    fn it_creates_a_user_and_finds_by_twitch_id() {
        let pool = setup_pool();
        let user = create_test_user(&pool);

        assert!(user.is_ok());
        let unwrapped = user.unwrap();

        let found_user = get_by_twitch_id(&pool, 8).unwrap();
        assert_eq!(unwrapped.name, found_user.name);
        assert_eq!(unwrapped.last_ip, found_user.last_ip);
        assert_eq!(unwrapped.service, found_user.service);
        assert_eq!(unwrapped.stream_path, found_user.stream_path);
    }

    #[test]
    fn it_creates_a_user_and_finds_by_name() {
        let pool = setup_pool();
        let user = create_test_user(&pool);

        assert!(user.is_ok());
        let unwrapped = user.unwrap();

        let found_user = get_by_name(&pool, "jbpratt").unwrap();
        assert_eq!(unwrapped.name, found_user.name);
        assert_eq!(unwrapped.last_ip, found_user.last_ip);
        assert_eq!(unwrapped.service, found_user.service);
        assert_eq!(unwrapped.stream_path, found_user.stream_path);
    }

    #[test]
    fn it_creates_a_user_and_finds_by_stream_path() {
        let pool = setup_pool();
        let user = create_test_user(&pool);

        assert!(user.is_ok());
        let unwrapped = user.unwrap();

        let found_user = get_by_stream_path(&pool, "/twitch/jbpratt").unwrap();
        assert_eq!(unwrapped.name, found_user.name);
        assert_eq!(unwrapped.last_ip, found_user.last_ip);
        assert_eq!(unwrapped.service, found_user.service);
        assert_eq!(unwrapped.stream_path, found_user.stream_path);
    }

    #[test]
    fn it_creates_a_user_and_updates() {
        let pool = setup_pool();
        let new_user = create_test_user(&pool);

        assert!(new_user.is_ok());
        let mut unwrapped = new_user.unwrap();
        unwrapped.is_admin = Some(true);

        let res = update(&pool, &unwrapped);
        assert!(res.is_ok());

        let id = Uuid::parse_str(unwrapped.id.as_str()).unwrap();
        let found_user = get_by_id(&pool, id).unwrap();
        assert!(found_user.is_admin.unwrap());
    }
}
