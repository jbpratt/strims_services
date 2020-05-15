use anyhow::{anyhow, Context};
use async_trait::async_trait;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json_schema::Schema;
use url::Url;

use std::convert::TryFrom;
use std::env;
use std::sync::Arc;

use crate::service::{Service, ServiceChannel, API};

// just using default parts needed for now
const URL: &str = "https://www.googleapis.com/youtube/v3/videos";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideosResult {
    items: Vec<Channel>,
    page_info: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    id: String,
    snippet: Snippet,
    content_details: ContentDetails,
    statistics: Statistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Snippet {
    title: String,
    thumbnails: Thumbnails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Thumbnails {
    medium: Medium,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Medium {
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContentDetails {
    content_rating: ContentRating,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContentRating {
    yt_rating: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Statistics {
    view_count: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    total_results: i64,
}

pub struct Youtube {
    client: Arc<Client>,
    url: String,
}

impl Youtube {
    fn with_token(mut self, token: String) -> Self {
        self.url = Url::parse_with_params(&self.url, &[("key", token)])
            .expect("failed to parse url with token")
            .to_string();
        self
    }
}

#[async_trait]
impl API for Youtube {
    async fn request<'a>(&mut self, url: &'a str) -> anyhow::Result<Response, reqwest::Error> {
        self.client.get(url).send().await
    }
}

#[async_trait]
impl Service<Youtube, Channel> for Youtube {
    fn new(client: Arc<Client>) -> Youtube {
        let token = env::var("YOUTUBE_TOKEN").expect("`YOUTUBE_TOKEN` set for authorization");
        Youtube {
            client,
            url: URL.to_string(),
        }
        .with_token(token)
    }

    fn validate_schema(data: &Value) -> Result<(), String> {
        let raw_schema = r#"
          {
            "type": "object",
            "properties": {
              "pageInfo": {
                "type": "object",
                "properties": {
                  "totalResults": {"type": "integer"}
                },
                "required": ["totalResults"]
              },
              "items": {
                "type": "array",
                "minItems": 1,
                "items": {
                  "type": "object",
                  "properties": {
                    "snippet": {
                      "type": "object",
                      "properties": {
                        "title": {"type": "string"},
                        "thumbnails": {
                          "type": "object",
                          "properties": {
                            "medium": {
                              "type": "object",
                              "properties": {
                                "url": {
                                  "type": "string",
                                  "format": "uri"
                                }
                              },
                              "required": ["url"]
                            }
                          },
                          "required": ["medium"]
                        }
                      },
                      "required": ["title", "thumbnails"]
                    },
                    "liveStreamingDetails": {
                      "type": "object",
                      "properties": {
                        "concurrentViewers": {
                          "type": "string",
                          "pattern": "^[0-9]+$"
                        }
                      }
                    },
                    "statistics": {
                      "type": "object",
                      "properties": {
                        "viewCount": {
                          "type": "string",
                          "pattern": "^[0-9]+$"
                        }
                      },
                      "required": ["viewCount"]
                    },
                    "contentDetails": {
                      "type": "object",
                      "properties": {
                        "contentRating": {
                          "type": "object",
                          "properties": {
                            "ytRating": {
                              "type": "string"
                            }
                          }
                        }
                      }
                    }
                  },
                  "required": ["snippet", "contentDetails"]
                }
              }
            }
          }
            "#;
        let schema = Schema::try_from(raw_schema).expect("failed to parse schema");
        schema.validate(data).map_err(|ss| ss.into_iter().collect())
    }

    async fn get_channel_by_name(&mut self, name: &str) -> anyhow::Result<Channel> {
        let parts = vec![
            "liveStreamingDetails",
            "snippet",
            "statistics",
            "contentDetails",
        ];

        let url = Url::parse_with_params(&self.url, &[("id", name), ("part", &parts.join(","))])?
            .to_string();

        let json_resp = self.request(&url).await?.json::<Value>().await?;

        match Youtube::validate_schema(&json_resp) {
            Ok(_) => {
                let results: VideosResult = serde_json::from_value(json_resp)?;
                return Ok(results.items[0].clone());
            }
            Err(e) => return Err(anyhow!("response failed validation: {}", e)),
        }
    }
}

impl ServiceChannel for Channel {
    fn get_live(&self) -> bool {
        true
    }
    fn is_nsfw(&self) -> bool {
        self.content_details.content_rating.yt_rating == "ytAgeRestricted"
    }
    fn get_title(&self) -> &str {
        self.snippet.title.as_str()
    }
    fn get_thumbnail(&self) -> &str {
        self.snippet.thumbnails.medium.url.as_str()
    }
    fn get_viewers(&self) -> u32 {
        self.statistics.view_count.parse::<u32>().unwrap()
    }
}
