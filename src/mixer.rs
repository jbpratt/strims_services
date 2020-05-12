use async_trait::async_trait;
use serde::Deserialize;
use reqwest::{Client, Response};
use std::sync::Arc;
use serde_json::Value;
use serde_json_schema::Schema;
use std::convert::TryFrom;

use crate::service::{API, Service};

#[derive(Deserialize)]
pub struct MixerChannel {
    pub name: String,
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
impl Service<Mixer, MixerChannel> for Mixer {
    fn new(client: Arc<Client>) -> Mixer {
        Mixer { client }
    }

    fn validate_schema(data: &Value) -> Result<(), Vec<String>> {
        let raw_schema: &str = r#"
          {
            "type": "object",
            "properties": {
              "views": {"type": "integer"},
              "preview": {
                "type": "object",
                "properties": {
                  "large": {
                    "type": "string",
                    "format": "uri"
                  }
                },
                "required": ["large"]
              },
              "title": {"type": "string"}
            },
            "required": ["views", "preview", "title"]
          }"#;
        // TODO: handle error here with `?`
        let schema = Schema::try_from(raw_schema).unwrap();
        schema.validate(&data)
    }

    async fn get_channel_by_name(&mut self) -> reqwest::Result<MixerChannel> {
        self.request("https://pastebin.com/raw/6Ux1nT23")
            .await?
            .json::<MixerChannel>()
            .await
    }
}
