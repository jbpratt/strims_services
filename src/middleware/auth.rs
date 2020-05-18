#![allow(dead_code)]
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::config::CONFIG;
use crate::errors::ApiError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateClaim {
    id: String,
    exp: i64,
}

impl PrivateClaim {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_ttl)).timestamp(),
        }
    }
}

pub fn encode_session_cookie(private_claim: PrivateClaim) -> Result<String, ApiError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| ApiError::CannotEncodeSessionToken(e.to_string()))
}

pub fn decode_session_cookie(token: &str) -> Result<PrivateClaim, ApiError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::CannotDecodeSessionToken(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes_a_session_cookie() {
        let private_claim = PrivateClaim::new("134109801571028");
        let jwt = encode_session_cookie(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_session_cookie() {
        let private_claim = PrivateClaim::new("134109801571028");
        let jwt = encode_session_cookie(private_claim.clone()).unwrap();
        let decoded = decode_session_cookie(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
