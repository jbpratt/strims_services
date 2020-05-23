use actix_identity::Identity;
use actix_web::{http, web, HttpRequest, HttpResponse};
use url::Url;

use crate::config::CONFIG;
use crate::database::DbPool;
use crate::errors::ApiError;
// streams
// login
// oauth
// admin/profiles/*/username
// admin/streams/**

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/login", web::get().to(login))
            .route("/oauth", web::get().to(oauth)),
    );
}

pub async fn oauth(req: HttpRequest) -> HttpResponse {
    dbg!(req);
    todo!()
}

pub async fn login(req: HttpRequest) -> HttpResponse {
    let url = "https://api.twitch.tv/kraken/oauth2/authorize?response_type=code&scope=user_read";
    let url = Url::parse_with_params(
        &url,
        &[
            ("client_id", CONFIG.twitch_client_id.clone()),
            ("redirect_uri", CONFIG.twitch_redirect_url.clone()),
        ],
    )
    .expect("failed to parse twitch login url")
    .to_string();

    HttpResponse::Found()
        .header(http::header::LOCATION, url)
        .finish()
        .into_body()
}
