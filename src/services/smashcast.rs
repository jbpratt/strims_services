use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::{RequestBuilder, Response};
use serde::Deserialize;
use serde_json::Value;

use std::sync::Arc;

use crate::service::{validate_schema, Service, ServiceChannel, API};

const URL: &str = "https://api.smashcast.tv/media/live/";

#[derive(Clone)]
pub struct Client {
    client: Arc<reqwest::Client>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideosResult {
    pub livestream: Vec<Channel>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(rename = "media_is_live")]
    pub media_is_live: String,
    // TODO: nsfw
    // #[serde(rename = "media_mature")]
    // pub media_mature: ::serde_json::Value,
    #[serde(rename = "media_status")]
    pub media_status: String,
    #[serde(rename = "media_views")]
    pub media_views: String,
    #[serde(rename = "media_thumbnail")]
    pub media_thumbnail: String,
}

#[async_trait]
impl API for Client {
    async fn request<'a>(&self, req: RequestBuilder) -> anyhow::Result<Response, reqwest::Error> {
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
              "livestream": {
                "type": "array",
                "items": {
                  "type": "object",
                  "properties": {
                    "media_status": {"type": "string"},
                    "media_is_live": {"type": "string"},
                    "media_thumbnail": {"type": "string"},
                    "media_views": {
                      "type": "string",
                      "pattern": "^[0-9]+$"
                    }
                  },
                  "required": [
                    "media_status",
                    "media_is_live",
                    "media_thumbnail",
                    "media_views"
                  ]
                },
                "minItems": 1
              }
            },
            "required": ["livestream"]
          }"#
    }

    async fn get_channel_by_name(&self, name: &str) -> anyhow::Result<Channel> {
        let url = URL.to_owned() + name;
        let json_resp = self
            .request(self.client.get(&url))
            .await?
            .json::<Value>()
            .await?;

        println!("{}", url);

        match validate_schema(&json_resp, Client::get_schema()) {
            Ok(_) => {
                let results: VideosResult = serde_json::from_value(json_resp)?;
                Ok(results.livestream[0].clone())
            }
            Err(e) => Err(anyhow!(
                "response failed validation: {} {}",
                json_resp.to_string(),
                e
            )),
        }
    }
}

impl ServiceChannel for Channel {
    fn get_live(&self) -> bool {
        self.media_is_live == "1"
    }
    fn is_nsfw(&self) -> bool {
        false
    }
    fn get_title(&self) -> String {
        self.media_status.clone()
    }
    fn get_thumbnail(&self) -> String {
        format!("https://edge.sf.hitbox.tv{}", self.media_thumbnail)
    }
    fn get_viewers(&self) -> u32 {
        self.media_views.parse::<u32>().unwrap()
    }
}
