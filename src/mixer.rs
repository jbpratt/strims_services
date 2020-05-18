use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;

use std::sync::Arc;

use crate::service::{validate_schema, Service, ServiceChannel, API};

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

#[derive(serde::Serialize, Deserialize, Debug)]
struct Thumbnail {
    url: String,
}

#[derive(Clone)]
pub struct Client {
    client: Arc<reqwest::Client>,
}

#[async_trait]
impl API for Client {
    async fn request<'a>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<Response, reqwest::Error> {
        let req = req.build()?;
        let resp = self.client.execute(req).await?;
        resp.error_for_status_ref()?;
        Ok(resp)
    }
}

#[async_trait]
impl Service<Channel> for Client {
    fn new(client: Arc<reqwest::Client>) -> Client {
        Client { client }
    }

    fn get_schema() -> &'static str {
        r#"
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
          }"#
    }

    async fn get_channel_by_name(&self, name: &str) -> anyhow::Result<Channel> {
        let url = URL.to_owned() + name;
        let json_resp = self
            .request(self.client.get(&url))
            .await?
            .json::<Value>()
            .await?;

        match validate_schema(&json_resp, Client::get_schema()) {
            Ok(_) => {
                let channel: Channel = serde_json::from_value(json_resp)?;
                return Ok(channel);
            }
            Err(e) => {
                return Err(anyhow!(
                    "response failed validation: {} {}",
                    json_resp.to_string(),
                    e
                ))
            }
        }
    }
}

impl ServiceChannel for Channel {
    fn get_live(&self) -> bool {
        self.online
    }
    fn is_nsfw(&self) -> bool {
        self.audience == "18+!"
    }
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn get_thumbnail(&self) -> String {
        self.thumbnail.url.clone()
    }
    fn get_viewers(&self) -> u32 {
        self.viewers_current
    }
}
