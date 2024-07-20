use serde::Deserialize;

use crate::model::auth::RequestCredential;
use crate::model::error::XError;
use crate::model::spaces::{Includes, Space};

#[derive(Debug, Deserialize)]
pub struct Response {
    pub data: Vec<Space>,
    pub includes: Option<Includes>,
}

#[derive(Clone, Debug)]
pub enum RateLimitContext {
    Application(String),
    AccessToken(RequestCredential),
}

impl Response {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, XError> {
        serde_json::from_slice::<Self>(bytes).map_err(|e| XError::Deserialize(e))
    }
}

impl<'a> super::Response<'a> for Response {}
