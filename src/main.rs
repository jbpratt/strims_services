#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
extern crate thiserror;

mod channel;
mod config;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;
mod server;
mod state;

mod mixer;
mod service;
mod smashcast;
mod twitch;
mod youtube;

use crate::server::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
