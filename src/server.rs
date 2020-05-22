use crate::{
    service,
    services::{mixer, smashcast, twitch, youtube},
    state,
};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use reqwest::Client;

use dotenv::dotenv;
use std::sync::Arc;

use crate::database::DbPool;
use crate::service::Service;

pub async fn server() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let port = 8080;

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let client = Arc::new(Client::new());
    let data = state::AppState {
        twitch: twitch::Client::new(client.clone()),
        mixer: mixer::Client::new(client.clone()),
        smashcast: smashcast::Client::new(client.clone()),
        youtube: youtube::Client::new(client.clone()),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(data.clone())
            .data(pool.clone())
            .service(no_params)
            .service(index)
    })
    .bind(("localhost", port))?
    .run()
    .await
}

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

#[get("/{service}/{name}")]
async fn index(
    info: web::Path<(String, String)>,
    data: web::Data<state::AppState>,
) -> Result<HttpResponse> {
    match info.0.as_str() {
        "twitch" => Ok(HttpResponse::NotFound().finish()),
        "mixer" => {
            let res = data
                .mixer
                .get_channel_by_name(info.1.as_str())
                .await
                .unwrap();
            Ok(HttpResponse::Ok().json(&res as &dyn service::ServiceChannel))
        }
        "smashcast" => {
            let res = data
                .smashcast
                .get_channel_by_name(info.1.as_str())
                .await
                .unwrap();
            Ok(HttpResponse::Ok().json(&res as &dyn service::ServiceChannel))
        }
        "youtube" => {
            let res = data
                .youtube
                .get_channel_by_name(info.1.as_str())
                .await
                .unwrap();
            Ok(HttpResponse::Ok().json(&res as &dyn service::ServiceChannel))
        }
        _ => Ok(HttpResponse::NotFound().finish()),
    }
}
