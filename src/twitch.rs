use async_trait::async_trait;
use reqwest::{header, Client, Response};
use serde::Deserialize;
use serde_json::Value;
use serde_json_schema::Schema;
use std::sync::Arc;

use crate::service::{Service, API};
use std::convert::TryFrom;
use std::env;

#[derive(Deserialize)]
pub struct Channel {
    pub username: String,
}

pub struct Twitch {
    client: Arc<Client>,
    token: String,
    client_id: String,
}

#[async_trait]
impl API for Twitch {
    async fn request<'a>(&mut self, url: &'a str) -> reqwest::Result<Response> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("OAuth {}", self.token).parse().unwrap(),
        );
        headers.insert("Client-ID", self.client_id.parse().unwrap());
        headers.insert(
            header::ACCEPT,
            "application/vnd.twitchtv.v5+json".parse().unwrap(),
        );
        self.client.get(url).headers(headers).send().await
    }
}

#[async_trait]
impl Service<Twitch, Channel> for Twitch {
    fn new(client: Arc<Client>) -> Twitch {
        let token = env::var("TWITCH_TOKEN").unwrap();
        let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
        Twitch {
            client,
            token,
            client_id,
        }
    }

    fn validate_schema(data: &Value) -> Result<(), Vec<String>> {
        let raw_schema = r#""#;
        let schema = Schema::try_from(raw_schema).unwrap();
        schema.validate(&data)
    }

    async fn get_channel_by_name(&mut self) -> reqwest::Result<Channel> {
        self.request("https://pastebin.com/raw/bPJnHaUt")
            .await?
            .json::<Channel>()
            .await
    }
}
