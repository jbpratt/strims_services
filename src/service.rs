use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Service<T, J> {
    fn new(client: Arc<reqwest::Client>) -> T;
    fn validate_schema(data: &serde_json::Value) -> anyhow::Result<(), String>;
    async fn get_channel_by_name(&mut self, name: &str) -> anyhow::Result<J>;
}

#[async_trait]
pub trait API {
    async fn request<'a>(&mut self, url: &'a str) -> anyhow::Result<reqwest::Response, reqwest::Error>;
}

pub trait ServiceChannel {
    fn get_live(&self) -> bool;
    fn is_nsfw(&self) -> bool;
    fn get_title(&self) -> &str;
    fn get_thumbnail(&self) -> &str;
    fn get_viewers(&self) -> u32;
}
