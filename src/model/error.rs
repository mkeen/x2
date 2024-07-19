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
    #[error("Generic: {0} {1}")]
    Generic(reqwest::StatusCode, String),
    #[error("Generic: {0}")]
    IO(io::Error),
}
