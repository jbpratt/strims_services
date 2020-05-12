use futures::try_join;
use reqwest::Client;

use std::sync::Arc;

mod mixer;
mod twitch;
mod service;

use crate::service::Service;

#[tokio::main]
async fn main() {
    let result = get_responses();
    let (res1, res2) = result.await.unwrap();
    println!("{}", res1.name);
    println!("{}", res2.username);
}

async fn get_responses() -> reqwest::Result<(mixer::MixerChannel, twitch::Channel)> {
    let client = Arc::new(Client::new());
    let mut mixer_client = mixer::Mixer::new(client.clone());
    let mut twitch_client = twitch::Twitch::new(client.clone());
    try_join!(
        mixer_client.get_channel_by_name(),
        twitch_client.get_channel_by_name()
    )
}
