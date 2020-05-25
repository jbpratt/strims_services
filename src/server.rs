use crate::{
    config::CONFIG,
    database::DbPool,
    routes::routes,
    service,
    service::Service,
    services::{mixer, smashcast, twitch, youtube},
    state,
    wsservice::ws_index,
};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use reqwest::Client;

use dotenv::dotenv;
use std::sync::Arc;

pub async fn server() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let port = 8080;

    let manager = ConnectionManager::<SqliteConnection>::new(CONFIG.database_url.clone());
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
            .service(index)
            .service(web::resource("/ws").route(web::get().to(ws_index)))
            .configure(routes)
    })
    .bind(("localhost", port))?
    .run()
    .await
}

#[get("/{service}/{name}")]
async fn index(
    info: web::Path<(String, String)>,
    data: web::Data<state::AppState>,
) -> actix_web::Result<HttpResponse> {
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
