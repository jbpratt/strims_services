//use futures::try_join;
use reqwest::Client;

use std::sync::Arc;

mod youtube;
mod mixer;
mod service;
mod twitch;

use crate::service::Service;

#[tokio::main]
async fn main() {
    /*
    let result = get_responses();
    let (res1, res2) = result.await.unwrap();
    println!("{}", res1.name);
    println!("{}", res2.username);
    */
    let client = Arc::new(Client::new());
    //let mut mixer_client = mixer::Mixer::new(client.clone());
    //let res = mixer_client.get_channel_by_name("ObiBertKenobi").await;
    let mut youtube_client = youtube::Youtube::new(client.clone());
    let res = youtube_client.get_channel_by_name("8pEpH1JWyiQ").await;
    println!("{:?}", res);
}

/*
async fn get_responses() -> reqwest::Result<(mixer::Channel, twitch::Channel)> {
    let client = Arc::new(Client::new());
    let mut mixer_client = mixer::Mixer::new(client.clone());
    let mut twitch_client = twitch::Twitch::new(client.clone());
    try_join!(
        mixer_client.get_channel_by_name("ObiBertKenobi"),
        twitch_client.get_channel_by_name("")
    )
}
*/
