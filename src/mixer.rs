use async_trait::async_trait;
use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::Value;
use serde_json_schema::Schema;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::service::{Service, ServiceChannel, API};

const URL: &'static str = "https://mixer.com/api/v1/channels/";

#[derive(Deserialize, Debug)]
pub struct Channel {
    #[serde(rename(deserialize = "name"))]
    title: String,
    online: bool,
    audience: String,
    #[serde(rename(deserialize = "viewersCurrent"))]
    viewers_current: u32,
    thumbnail: Thumbnail,
}

#[derive(Deserialize, Debug)]
struct Thumbnail {
    url: String
}

pub struct Mixer {
    client: Arc<Client>,
}

#[async_trait]
impl API for Mixer {
    async fn request<'a>(&mut self, url: &'a str) -> reqwest::Result<Response> {
        self.client.get(url).send().await
    }
}

#[async_trait]
impl Service<Mixer, Channel> for Mixer {
    fn new(client: Arc<Client>) -> Mixer {
        Mixer { client }
    }

    fn validate_schema(data: Value) -> Result<(), Vec<String>> {
        let raw_schema: &str = r#"
          {
            "type": "object",
            "properties": {
              "name": {"type": "string"},
              "audience": {"type": "string"},
              "online": {"type": "boolean"},
              "thumbnail": {
                "type": "object",
                "properties": {
                  "url": {
                    "type": "string",
                    "format": "uri"
                  },
                  "required": ["url"]
                }
              },
              "viewersCurrent": {"type": "integer"}
            },
            "required": ["name", "online", "thumbnail", "viewersCurrent", "audience"]
          }"#;
        // TODO: handle error here with `?`
        let schema = Schema::try_from(raw_schema).unwrap();
        schema.validate(&data)
    }

    async fn get_channel_by_name(&mut self, name: &str) -> reqwest::Result<Channel> {
        let url = URL.to_owned() + name;
        self.request(&url).await?.json::<Channel>().await
    }
}

impl ServiceChannel for Channel {
    fn get_live(&self) -> bool {
        self.online
    }
    fn is_nsfw(&self) -> bool {
        self.audience == "18+!"
    }
    fn get_title(&self) -> &str {
        self.title.as_str()
    }
    fn get_thumbnail(&self) -> &str {
        self.thumbnail.url.as_str()
    }
    fn get_viewers(&self) -> u32 {
        self.viewers_current
    }
}
