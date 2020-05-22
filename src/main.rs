#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel_migrations;
extern crate thiserror;

mod channel;
mod config;
mod database;
mod errors;
mod helpers;
mod middleware;
mod models;
mod routes;
mod schema;
mod server;
mod state;

mod service;
mod services;

use crate::server::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
