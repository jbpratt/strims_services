use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use std::sync::Arc;

use crate::config::CONFIG;
use crate::service::{validate_schema, Service, ServiceChannel, API};

// just using default parts needed for now
const URL: &str = "https://www.googleapis.com/youtube/v3/videos";

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideosResult {
    items: Vec<Channel>,
    page_info: PageInfo,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    id: String,
    snippet: Snippet,
    content_details: ContentDetails,
    statistics: Statistics,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
struct Snippet {
    title: String,
    thumbnails: Thumbnails,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
struct Thumbnails {
    medium: Medium,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
struct Medium {
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContentDetails {
    content_rating: ContentRating,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContentRating {
    #[serde(default)]
    yt_rating: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Statistics {
    view_count: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    total_results: i64,
}

#[derive(Clone)]
pub struct Client {
    client: Arc<reqwest::Client>,
    url: String,
}

impl Client {
    fn with_token(mut self, token: String) -> Self {
        self.url = Url::parse_with_params(&self.url, &[("key", token)])
            .expect("failed to parse url with token")
            .to_string();
        self
    }
}

#[async_trait]
impl API for Client {
    async fn request<'a>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<Response, reqwest::Error> {
        log::info!("Making request: {:?}", req);
        let req = req.build()?;
        let resp = self.client.execute(req).await?;
        resp.error_for_status_ref()?;
        log::debug!("{:?}", resp);
        Ok(resp)
    }
}

#[async_trait]
impl Service<Channel> for Client {
    fn new(client: Arc<reqwest::Client>) -> Client {
        Client {
            client,
            url: URL.to_string(),
        }
        .with_token(CONFIG.youtube_token.clone())
    }

    fn get_schema() -> &'static str {
        r#"
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
        "#
    }

    async fn get_channel_by_name(&self, name: &str) -> anyhow::Result<Channel> {
        let parts = vec![
            "liveStreamingDetails",
            "snippet",
            "statistics",
            "contentDetails",
        ];

        let json_resp = self
            .request(
                self.client
                    .get(&self.url)
                    .query(&[("id", name), ("part", &parts.join(","))]),
            )
            .await?
            .json::<Value>()
            .await?;

        match validate_schema(&json_resp, Client::get_schema()) {
            Ok(_) => {
                let results: VideosResult = serde_json::from_value(json_resp)?;
                Ok(results.items[0].clone())
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
        true
    }
    fn is_nsfw(&self) -> bool {
        self.content_details.content_rating.yt_rating == "ytAgeRestricted"
    }
    fn get_title(&self) -> String {
        self.snippet.title.clone()
    }
    fn get_thumbnail(&self) -> String {
        self.snippet.thumbnails.medium.url.clone()
    }
    fn get_viewers(&self) -> u32 {
        self.statistics.view_count.parse::<u32>().unwrap()
    }
}
