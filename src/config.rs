use std::env;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub youtube_token: String,
    pub twitch_token: String,
    pub twitch_client_id: String,
    pub jwt_key: String,
    pub jwt_ttl: i64,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    dotenv::dotenv().ok();

    let youtube_token = env::var("YOUTUBE_TOKEN").expect("`YOUTUBE_TOKEN` set for authorization");
    let twitch_token = env::var("TWITCH_TOKEN").expect("`TWITCH_TOKEN` set for authorization");
    let twitch_client_id =
        env::var("TWITCH_CLIENT_ID").expect("`TWITCH_CLIENT_ID` set for authorization");
    let jwt_key = env::var("JWT_KEY").expect("JWT_KEY");
    let jwt_ttl = env::var("JWT_TTL")
        .expect("JWT_TTL")
        .parse::<i64>()
        .expect("i64 for JWT_TTL");

    Config {
        youtube_token,
        twitch_token,
        twitch_client_id,
        jwt_key,
        jwt_ttl,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = get_config();
        assert_ne!(config.jwt_key, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.jwt_key, "".to_string());
    }
}
