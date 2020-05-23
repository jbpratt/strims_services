use anyhow::Context;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{header, Response};
use serde::Deserialize;
use std::sync::Arc;

use crate::config::CONFIG;
use crate::service::{Service, ServiceChannel, API};

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

#[derive(Clone)]
pub struct Client {
    client: Arc<reqwest::Client>,
    token: String,
    client_id: String,
    //client_secret: String,
}

#[async_trait]
impl API for Client {
    async fn request<'a>(
        &self,
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
impl Service<Channel> for Client {
    fn new(client: Arc<reqwest::Client>) -> Client {
        Client {
            client,
            token: CONFIG.twitch_client_secret.clone(),
            client_id: CONFIG.twitch_client_id.clone(),
        }
    }

    fn get_schema() -> &'static str {
        r#"
            "#
    }

    async fn get_channel_by_name(&self, name: &str) -> anyhow::Result<Channel> {
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
