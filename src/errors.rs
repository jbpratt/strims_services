#![allow(dead_code)]
use diesel::{r2d2, result::Error as DBError};
use thiserror::Error;

use std::fmt;

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("database error: {0}")]
    DatabaseError(#[from] DBError),
    #[error("pool error: {0}")]
    PoolError(#[from] PoolError),
    #[error("failed to encode session token: {0}")]
    CannotEncodeSessionToken(String),
    #[error("failed to decode session token: {0}")]
    CannotDecodeSessionToken(String),
    #[error("response failed schema validation: {0}")]
    SchemaValidation(String),
    #[error("channel failed validation: {0}")]
    ChannelValidation(String),
    #[error("channel failed normaliztion: {0}")]
    ChannelNormalization(#[from] url::ParseError),
    #[error("not found")]
    NotFound(String),
    #[error("failed to parse uuid: {0}")]
    CannotParseUuid(String),
    #[error("failed to parse ipaddr: {0}")]
    CannotParseIPAddr(String),
    #[error("invalid service: {0}")]
    InvalidService(String),
}

#[derive(Debug, Error)]
pub struct PoolError(#[source] r2d2::PoolError);
impl PartialEq for PoolError {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<r2d2::PoolError> for ApiError {
    fn from(e: r2d2::PoolError) -> Self {
        ApiError::PoolError(PoolError(e))
    }
}
