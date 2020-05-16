use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Result};
use reqwest::Client;

use dotenv::dotenv;
use std::sync::Arc;

mod mixer;
mod service;
mod smashcast;
mod twitch;
mod youtube;

#[derive(Clone)]
struct AppState {
    twitch: twitch::Client,
    mixer: mixer::Client,
    smashcast: smashcast::Client,
    youtube: youtube::Client,
}

use crate::service::Service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let port = 8080;

    let client = Arc::new(Client::new());
    let data = AppState {
        twitch: twitch::Client::new(client.clone()),
        mixer: mixer::Client::new(client.clone()),
        smashcast: smashcast::Client::new(client.clone()),
        youtube: youtube::Client::new(client.clone()),
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(data.clone())
            .service(no_params)
            .service(index)
    })
    .bind(("localhost", port))?
    .run();

    eprintln!("Listening on localhost:{}", port);
    server.await
}

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

#[get("/{service}/{name}")]
async fn index(
    info: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    match info.0.as_str() {
        "twitch" => Ok(HttpResponse::NotFound().finish()),
        "mixer" => {
            let res = data
                .mixer
                .get_channel_by_name(info.1.as_str())
                .await
                .unwrap();
            return Ok(HttpResponse::Ok().json(&res as &dyn service::ServiceChannel));
        }
        "smashcast" => {
            let res = data
                .smashcast
                .get_channel_by_name(info.1.as_str())
                .await
                .unwrap();
            return Ok(HttpResponse::Ok().json(&res as &dyn service::ServiceChannel));
        }
        "youtube" => {
            let res = data
                .youtube
                .get_channel_by_name(info.1.as_str())
                .await
                .unwrap();
            return Ok(HttpResponse::Ok().json(&res as &dyn service::ServiceChannel));
        }
        _ => return Ok(HttpResponse::NotFound().finish()),
    }
}
