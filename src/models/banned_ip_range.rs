use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use std::net::IpAddr;

use crate::database::DbPool;
use crate::errors::ApiError;
use crate::schema::banned_ip_ranges;

#[derive(Queryable, Debug, Identifiable, Insertable, Clone, Deserialize, Serialize)]
#[primary_key(start, end)]
pub struct BannedIpRange {
    pub start: String,
    pub end: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for BannedIpRange {
    fn default() -> Self {
        Self {
            start: String::new(),
            end: String::new(),
            note: Some(String::new()),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

pub fn get_all(pool: &DbPool) -> anyhow::Result<Vec<BannedIpRange>, ApiError> {
    let conn = pool.get()?;
    let all = banned_ip_ranges::table.load(&conn)?;
    Ok(all)
}

pub fn insert(pool: &DbPool, range: &BannedIpRange) -> anyhow::Result<BannedIpRange, ApiError> {
    let conn = pool.get()?;
    diesel::insert_into(banned_ip_ranges::table)
        .values(range)
        .execute(&conn)?;
    Ok(range.clone())
}

pub fn remove(pool: &DbPool, range: &BannedIpRange) -> anyhow::Result<(), ApiError> {
    use crate::schema::banned_ip_ranges::dsl::{end, start};

    let conn = pool.get()?;
    diesel::delete(
        banned_ip_ranges::table
            .filter(start.eq(range.start.clone()))
            .filter(end.eq(range.end.clone())),
    )
    .execute(&conn)?;

    Ok(())
}

pub fn check_if_banned(
    pool: &DbPool,
    addr: &str,
) -> anyhow::Result<Option<BannedIpRange>, ApiError> {
    let ranges = get_all(&pool)?;
    let result = parse_ip(addr)?;

    for range in ranges {
        let start = parse_ip(range.start.as_str())?;
        let end = parse_ip(range.end.as_str())?;
        if start < result && result < end {
            return Ok(Some(range));
        }
    }

    Ok(None)
}

// Parse an IP address and return a Vec<u8> of octets
fn parse_ip(ip_addr: &str) -> anyhow::Result<Vec<u8>, ApiError> {
    let check_ip: IpAddr = ip_addr
        .parse()
        .map_err(|_| ApiError::CannotParseIPAddr(ip_addr.to_string()))?;

    match check_ip {
        IpAddr::V4(ip) => Ok(ip.octets().to_vec()),
        IpAddr::V6(ip) => Ok(ip.octets().to_vec()),
    }
}

mod tests {
    use super::*;
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::SqliteConnection;
    embed_migrations!();

    fn setup_pool() -> DbPool {
        let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool: DbPool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");

        embedded_migrations::run(&pool.get().unwrap()).expect("failed to run migrations");
        pool
    }

    #[test]
    fn it_inserts_a_banned_ip_range() {
        let pool = setup_pool();
        let banned_ip_range = BannedIpRange {
            start: String::from("127.0.0.25"),
            end: String::from("127.0.0.50"),
            note: Some(String::from("duckerz")),
            ..Default::default()
        };

        let created = insert(&pool, &banned_ip_range.clone());
        assert!(created.is_ok());

        let ip_range = created.unwrap();

        assert_eq!(banned_ip_range.start, ip_range.start);
        assert_eq!(banned_ip_range.end, ip_range.end);
    }

    #[test]
    fn it_inserts_and_gets_all_banned_ip_ranges() {
        let pool = setup_pool();
        let banned_ip_range_1 = BannedIpRange {
            start: String::from("127.0.0.25"),
            end: String::from("127.0.0.50"),
            note: Some(String::from("duckerz")),
            ..Default::default()
        };
        let banned_ip_range_2 = BannedIpRange {
            start: String::from("127.0.0.25"),
            end: String::from("127.0.0.100"),
            note: Some(String::from("lewd posters out")),
            ..Default::default()
        };

        let _ = insert(&pool, &banned_ip_range_1.clone());
        let _ = insert(&pool, &banned_ip_range_2.clone());

        let ranges = get_all(&pool);
        assert!(ranges.is_ok());

        let ranges = ranges.unwrap();
        assert_eq!(ranges.len(), 2);
    }

    #[test]
    fn it_checks_and_doesnt_find_a_banned_ip_range() {
        let pool = setup_pool();
        let results = check_if_banned(&pool, "10.0.0.16");
        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(results.is_none());
    }

    #[test]
    fn it_inserts_checks_and_finds_a_banned_ip_range() {
        let pool = setup_pool();
        let banned_ip_range = BannedIpRange {
            start: String::from("127.0.0.25"),
            end: String::from("127.0.0.50"),
            note: Some(String::from("duckerz")),
            ..Default::default()
        };

        let created = insert(&pool, &banned_ip_range.clone());
        assert!(created.is_ok());

        let check = check_if_banned(&pool, "127.0.0.30");
        assert!(check.is_ok());

        let check = check.unwrap();
        assert!(check.is_some());
    }

    #[test]
    fn it_inserts_v6_and_removes_a_banned_ip_range() {
        let pool = setup_pool();
        let banned_ip_range = BannedIpRange {
            start: String::from("2001:0000:0000:0000:0000:0000:0000:0000"),
            end: String::from("2001:0000:0000:0000:ffff:0000:0000:0000"),
            note: Some(String::from("duckerz")),
            ..Default::default()
        };

        let created = insert(&pool, &banned_ip_range.clone());
        assert!(created.is_ok());

        let removed = remove(&pool, &banned_ip_range.clone());
        assert!(removed.is_ok());
    }
}
