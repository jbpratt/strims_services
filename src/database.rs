use diesel::prelude::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbPooledConn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;
