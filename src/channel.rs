use regex::Regex;
use url::Url;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::errors::ApiError;

const SERVICES: [&str; 11] = [
    "advanced",
    "angelthump",
    "facebook",
    "m3u8",
    "smashcast",
    "twitch",
    "twitch-vod",
    "ustream",
    "vaughn",
    "youtube",
    "youtube-playlist",
];

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Channel {
    pub channel: String,
    pub service: String,
    pub stream_path: String,
}

impl Channel {
    pub fn new(
        channel: String,
        service: String,
        stream_path: String,
    ) -> anyhow::Result<Self, ApiError> {
        // validate service
        if !valid_service(service.as_str()) {
            return Err(ApiError::ChannelValidation(format!(
                "invalid service: {}",
                service
            )));
        }
        // normalize channel
        let normalized_channel = match normalize_channel(service.as_str(), channel.as_str()) {
            Ok(chn) => chn,
            Err(e) => return Err(e),
        };

        // validate path
        if !valid_stream_path(stream_path.as_str()) {
            return Err(ApiError::ChannelValidation(format!(
                "invalid stream path: {}",
                stream_path
            )));
        }

        let mut chn = Self {
            channel: normalized_channel,
            service,
            stream_path,
        };

        chn.stream_path = chn.get_path();

        Ok(chn)
    }

    fn get_path(&self) -> String {
        if !self.stream_path.is_empty() {
            self.stream_path.clone()
        } else {
            format!("/{}/{}", self.service, self.channel)
        }
    }
}
pub fn get_channel_id(chn: &Channel) -> u64 {
    &calc_channel_hash(chn) & 1 << (48 - 1)
}

fn calc_channel_hash(chn: &Channel) -> u64 {
    let mut hasher = DefaultHasher::new();
    chn.hash(&mut hasher);
    hasher.finish()
}

pub fn valid_service(service: &str) -> bool {
    SERVICES.contains(&service)
}

fn valid_stream_path(path: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-z0-9_]{3,32}$").unwrap();
    }
    if path.is_empty() {
        true
    } else {
        RE.is_match(path)
    }
}

fn valid_basic_channel(channel: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9\\-_]{1,64}$").unwrap();
    }
    RE.is_match(channel)
}

fn normalize_channel(service: &str, channel: &str) -> anyhow::Result<String, ApiError> {
    // advanced
    if service == "advanced" || service == "m3u8" {
        let channel_uri = Url::parse(channel)?;
        if channel_uri.scheme() != "http" && channel_uri.scheme() != "https" {
            return Err(ApiError::ChannelValidation(String::from(
                "invalid advanced url schema. must be http or https",
            )));
        }
        Ok(channel_uri.to_string())
    } else {
        if !valid_basic_channel(channel) {
            return Err(ApiError::ChannelValidation(String::from("invalid channel")));
        }

        if valid_service(channel) {
            return Ok(channel.to_ascii_lowercase());
        }
        Ok(channel.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_creates_a_valid_channel() {
        let response = Channel::new(
            String::from("jbpratt"),
            String::from("twitch"),
            String::from(""),
        )
        .unwrap();
        assert_eq!(response.stream_path, "/twitch/jbpratt");
    }

    #[test]
    fn it_fails_to_create_an_invalid_channel_service() {
        let response = Channel::new(
            String::from("jbpratt"),
            String::from("chaturbate"),
            String::from(""),
        );

        let expected_err = ApiError::ChannelValidation(String::from("invalid service: chaturbate"));

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_err);
    }

    #[test]
    fn it_creates_a_valid_advanced_channel() {
        let response = Channel::new(
            String::from(
                "https://api.new.livestream.com/accounts/1181452/events/8865379/live.m3u8",
            ),
            String::from("advanced"),
            String::from(""),
        )
        .unwrap();
        assert_eq!(
            response.get_path(),
            "/advanced/https://api.new.livestream.com/accounts/1181452/events/8865379/live.m3u8"
        );
    }

    #[test]
    fn it_fails_to_create_an_invalid_channel_advanced() {
        let response = Channel::new(
            String::from("m3u8://api.new.livestream.com/accounts/1181452/events/8865379/live"),
            String::from("advanced"),
            String::from(""),
        );
        let expected_err = ApiError::ChannelValidation(String::from(
            "invalid advanced url schema. must be http or https",
        ));

        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_err);
    }

    #[test]
    fn it_hashes_a_channel() {
        let response = Channel::new(
            String::from("test1"),
            String::from("twitch"),
            String::from("test2"),
        )
        .unwrap();

        let chn_hash = get_channel_id(&response);
        assert_eq!(chn_hash, 140737488355328);
    }
}
