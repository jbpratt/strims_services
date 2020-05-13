use async_trait::async_trait;
use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::Value;
use serde_json_schema::Schema;
use url::Url;

use std::convert::TryFrom;
use std::env;
use std::sync::Arc;

use crate::service::{Service, ServiceChannel, API};

// just using default parts needed for now
const URL: &str = "https://www.googleapis.com/youtube/v3/videos";

#[derive(Deserialize, Debug)]
pub struct Channel {
    title: String,
    viewers: u32,
    thumbnail: String,
    nsfw: bool,
}

pub struct Youtube {
    client: Arc<Client>,
    url: String,
}

impl Youtube {
    fn with_token(self, token: String) -> Self {
        Url::parse_with_params(&self.url, &[("key", token)])
            .unwrap()
            .to_string();
        self
    }
}

#[async_trait]
impl API for Youtube {
    async fn request<'a>(&mut self, url: &'a str) -> reqwest::Result<Response> {
        self.client.get(url).send().await
    }
}

#[async_trait]
impl Service<Youtube, Channel> for Youtube {
    fn new(client: Arc<Client>) -> Youtube {
        let token = env::var("YOUTUBE_TOKEN").unwrap();
        Youtube {
            client,
            url: URL.to_string(),
        }
        .with_token(token)
    }

    fn validate_schema(data: Value) -> Result<(), Vec<String>> {
        let raw_schema = r#""#;
        let schema = Schema::try_from(raw_schema).unwrap();
        schema.validate(&data)
    }

    async fn get_channel_by_name(&mut self, name: &str) -> reqwest::Result<Channel> {
        let parts = vec![
            "liveStreamingDetails",
            "snippet",
            "statistics",
            "contentDetails",
        ];
        let url = Url::parse_with_params(URL, &[("id", name)])
            .unwrap()
            .to_string();
        let url = Url::parse_with_params(&url, &[("part", parts.join(","))])
            .unwrap()
            .to_string();

        self.request(&url).await?.json::<Channel>().await
    }
}

impl ServiceChannel for Channel {
    fn get_live(&self) -> bool {
        true
    }
    fn is_nsfw(&self) -> bool {
        self.nsfw
    }
    fn get_title(&self) -> &str {
        self.title.as_str()
    }
    fn get_thumbnail(&self) -> &str {
        self.thumbnail.as_str()
    }
    fn get_viewers(&self) -> u32 {
        self.viewers
    }
}
