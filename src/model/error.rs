use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum XError {
    #[error("X Authentication Error: {0}")]
    Auth(reqwest::Error),
    #[error("Unknown error {0}")]
    Unknown(String),
    #[error("Lib error: {0}")]
    Lib(String),
    #[error("Lib error: {0}")]
    ParseResponseFailed(String),
    #[error("Panic: {0}")]
    Panic(serde_json::Error),
    #[error("HTTP Error: {0} {1}")]
    Generic(reqwest::StatusCode, String),
    #[error("IO: {0}")]
    IO(io::Error),
    #[error("Deserialization Error: {0}")]
    Deserialize(serde_json::Error),
    #[error("Reqwest Error: {0}")]
    Reqwest(reqwest::Error),
}
