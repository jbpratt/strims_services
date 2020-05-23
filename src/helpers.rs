use actix_web::{
    body::Body,
    web::{HttpResponse, Json},
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use lazy_static::lazy_static;
use serde::Serialize;

use crate::database::DbPool;
use crate::errors::ApiError;

pub fn setup_pool() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    embed_migrations!();
    embedded_migrations::run(&pool.get().unwrap()).expect("failed to run migrations");
    pool
}

pub fn respond_json<T>(data: T) -> anyhow::Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

pub fn respond_ok() -> anyhow::Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    struct TestResponse {
        name: String,
    }

    #[test]
    fn it_responds_ok() {
        let result = respond_ok();
        assert!(result.is_ok());
    }

    #[test]
    fn it_responds_json() {
        let res = TestResponse {
            name: "jbpratt".into(),
        };
        let result = respond_json(res.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), res);
    }
}
