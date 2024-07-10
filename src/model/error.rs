use thiserror::Error;

#[derive(Error, Debug)]
pub enum XError {
    #[error("X Authentication Error: {0}")]
    Auth(String),
    #[error("No hander for: {0}")]
    UnknownResponse(String),
    #[error("Lib error: {0}")]
    Lib(String),
}
