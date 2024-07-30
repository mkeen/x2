use std::io;

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
    #[error("Reqwest Error: {0}")]
    Reqwest(reqwest::Error),
    #[error("Invalid UTF8")]
    InvalidUtf8Response,
    #[error("Socket Error: {0}")]
    Socket(String),
    #[error("Http Error: {0}")]
    HttpGeneric(reqwest::StatusCode, String),
    #[error("Already Authorized")]
    AlreadyAuthorized,
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
