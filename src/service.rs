use async_trait::async_trait;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use std::sync::Arc;

#[async_trait]
pub trait Service<J> {
    fn new(client: Arc<reqwest::Client>) -> Self;
    fn validate_schema(data: &serde_json::Value) -> anyhow::Result<(), String>;
    async fn get_channel_by_name(&self, name: &str) -> anyhow::Result<J>;
}

#[async_trait]
pub trait API {
    async fn request<'a>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<reqwest::Response, reqwest::Error>;
}

pub trait ServiceChannel {
    fn get_live(&self) -> bool;
    fn is_nsfw(&self) -> bool;
    fn get_title(&self) -> String;
    fn get_thumbnail(&self) -> String;
    fn get_viewers(&self) -> u32;
}

impl Serialize for dyn ServiceChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("channel", 5)?;
        state.serialize_field("online", &self.get_live())?;
        state.serialize_field("nsfw", &self.is_nsfw())?;
        state.serialize_field("title", &self.get_title())?;
        state.serialize_field("thumbnail", &self.get_thumbnail())?;
        state.serialize_field("viewers", &self.get_viewers())?;
        state.end()
    }
}
