use std::{io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum XError {
    #[error("X Authentication Error: {0}")]
    Http(reqwest::Error),
    #[error("X Authentication Error: {0}")]
    Auth(XAuthError),
    #[error("Unknown error {0}")]
    Unknown(String),
    #[error("Lib error: {0}")]
    Lib(String),
    #[error("Lib error: {0}")]
    ParseResponseFailed(String),
    #[error("Panic: {0}")]
    Panic(serde_json::Error),
    #[error("IO: {0}")]
    IO(io::Error),
    #[error("Deserialization Error: {0}")]
    Deserialize(serde_json::Error),
    #[error("Response bytes read Error: {0}")]
    ReadResponseBytes(reqwest::Error),
    #[error("Reqwest Error: {0}")]
    Reqwest(reqwest::Error),
    #[error("Reqwest oauth1 Error: {0}")]
    ReqwestOauth1(reqwest_oauth1::Error),
    #[error("Invalid UTF8")]
    InvalidUtf8Response,
    #[error("Socket Error: {0}")]
    Socket(String),
    #[error("Http Error: {0}")]
    HttpGeneric(reqwest::StatusCode, String),
    #[error("Already Authorized")]
    AlreadyAuthorized,
    #[error("Already Consumed")]
    AlreadyConsumed,
}

#[derive(Error, Debug)]
pub enum XAuthError {
    #[error("no auth keys supplied")]
    NoKeys,
    #[error("Failed: {0}")]
    Upstream(String),
    #[error("Please file a bug report: https://github.com/mkeen/x2/issues")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum LibError {
    #[error("Please file a bug report: https://github.com/mkeen/x2/issues")]
    OutOfBounds,
    #[error("{0} Please file a bug report: https://github.com/mkeen/x2/issues")]
    UrlParsingError(String),
    #[error("Invalid snowflake specified: {0}")]
    InvalidSnowflake(ParseIntError),
    #[error("Invalid key for request type")]
    InvalidKeyForRequestType,
}
