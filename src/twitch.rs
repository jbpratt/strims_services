use anyhow::Context;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{header, Client, Response};
use serde::Deserialize;
use serde_json::Value;
use serde_json_schema::Schema;
use std::sync::Arc;

use crate::service::{Service, ServiceChannel, API};
use std::convert::TryFrom;
use std::env;

const URL: &str = "https://api.twitch.tv/helix/";

#[derive(Deserialize, Debug)]
pub struct Channel {
    game: String,
    viewers: u32,
    preview: String,
    #[serde(rename(deserialize = "channel"))]
    name: Name,
}

#[derive(Deserialize, Debug)]
struct Name {
    display_name: String,
}

pub struct Twitch {
    client: Arc<Client>,
    token: String,
    client_id: String,
    //client_secret: String,
}

#[async_trait]
impl API for Twitch {
    async fn request<'a>(
        &mut self,
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<Response, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        headers.insert("Client-ID", self.client_id.parse().unwrap());
        headers.insert(
            header::ACCEPT,
            "application/vnd.twitchtv.v5+json".parse().unwrap(),
        );
        let req = req.headers(headers).build()?;
        let resp = self.client.execute(req).await?;

        resp.error_for_status_ref()?;
        Ok(resp)
    }
}

#[async_trait]
impl Service<Channel> for Twitch {
    fn new(client: Arc<Client>) -> Twitch {
        let token = env::var("TWITCH_TOKEN").unwrap();
        let client_id = env::var("TWITCH_CLIENT_ID").unwrap();
        //let client_secret = env::var("TWITCH_CLIENT_SECRET").unwrap();
        Twitch {
            client,
            token,
            client_id,
            //   client_secret,
        }
    }

    fn validate_schema(data: &Value) -> Result<(), String> {
        let raw_schema = r#""#;
        let schema = Schema::try_from(raw_schema).unwrap();
        schema.validate(data).map_err(|ss| ss.into_iter().collect())
    }

    async fn get_channel_by_name(&mut self, name: &str) -> anyhow::Result<Channel> {
        let url = URL.to_owned() + name;
        self.request(self.client.get(&url))
            .await?
            .json::<Channel>()
            .await
            .context("failed to make request")
    }
}

impl ServiceChannel for Channel {
    fn get_live(&self) -> bool {
        true
    }
    fn is_nsfw(&self) -> bool {
        false
    }
    fn get_title(&self) -> String {
        self.game.clone()
    }
    fn get_thumbnail(&self) -> String {
        self.preview.clone()
    }
    fn get_viewers(&self) -> u32 {
        self.viewers
    }
}
