use anyhow::Result;
use reqwest::Client;
#[macro_use]
use serde::Deserialize;
use async_trait::async_trait;

trait Channel {
    fn get_live(&self) -> bool;
    fn is_nsfw(&self) -> bool;
    fn get_title(&self) -> &'static str;
    fn get_thumbnail(&self) -> &'static str;
    fn get_viewers(&self) -> u32;
}


#[async_trait]
trait Service<T> {
    async fn get_channel_by_name(&self, name: &str) -> Result<T>;
}

struct MixerClient {
    client: Client,
}

impl MixerClient {
    fn new() -> Result<Self> {
        Ok(Self {
            client: Client::new(),
        })
    }
}

#[async_trait]
impl Service<Mixer> for MixerClient {
    async fn get_channel_by_name(&self, name: &str) -> Result<Mixer> {
        let req_url = format!("https://mixer.com/api/v1/channels/{name}", name = name);
        let mut resp = self.client.get(&req_url).await;
        Ok(Mixer {
            live: false,
            nsfw: false,
            thumbnail: "",
            title: "",
            viewers: 0,
        })
    }
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
struct Mixer {
    live: bool,
    nsfw: bool,
    title: &'static str,
    thumbnail: &'static str,
    viewers: u32,
}

impl Channel for Mixer {
    fn get_live(&self) -> bool {
        self.live
    }

    fn is_nsfw(&self) -> bool {
        self.nsfw
    }

    fn get_title(&self) -> &'static str {
        self.title
    }

    fn get_thumbnail(&self) -> &'static str {
        self.thumbnail
    }

    fn get_viewers(&self) -> u32 {
        self.viewers
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
