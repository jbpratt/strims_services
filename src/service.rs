use reqwest::{Client, Response};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

#[async_trait]
pub trait Service<T, J> {
    fn new(client: Arc<Client>) -> T;
    fn validate_schema(data: &Value) -> Result<(), Vec<String>>;
    async fn get_channel_by_name(&mut self) -> reqwest::Result<J>;
}

#[async_trait]
pub trait API {
    async fn request<'a>(&mut self, url: &'a str) -> reqwest::Result<Response>;
}
