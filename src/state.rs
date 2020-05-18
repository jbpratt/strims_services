use crate::{mixer, smashcast, twitch, youtube};

#[derive(Clone)]
pub struct AppState {
    pub twitch: twitch::Client,
    pub mixer: mixer::Client,
    pub smashcast: smashcast::Client,
    pub youtube: youtube::Client,
}
