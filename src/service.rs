use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Service<J> {
    fn new(client: Arc<reqwest::Client>) -> Self;
    fn validate_schema(data: &serde_json::Value) -> anyhow::Result<(), String>;
    async fn get_channel_by_name(&mut self, name: &str) -> anyhow::Result<J>;
}

#[async_trait]
pub trait API {
    async fn request<'a>(
        &mut self,
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::Response, reqwest::Error>;
}

pub trait ServiceChannel {
    fn get_live(&self) -> bool;
    fn is_nsfw(&self) -> bool;
    fn get_title(&self) -> String;
    fn get_thumbnail(&self) -> String;
    fn get_viewers(&self) -> u32;
    fn display(&self) {
        println!(
            "Live: {} | Nsfw: {} | Title: {} | Thumbnail: {} | Viewers: {}",
            self.get_live(),
            self.is_nsfw(),
            self.get_title(),
            self.get_thumbnail(),
            self.get_viewers()
        )
    }
}
