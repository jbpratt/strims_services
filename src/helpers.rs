use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

use crate::channel::Channel;
use crate::database::DbPool;

embed_migrations!();

pub fn setup_pool() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    embedded_migrations::run(&pool.get().unwrap()).expect("failed to run migrations");
    pool
}
