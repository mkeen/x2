use serde::Deserialize;

use crate::model::auth::RequestCredential;
use crate::model::error::XError;
use crate::model::spaces::{Includes, Space};

pub use super::Response as ResponseTrait;

#[derive(Debug, Deserialize)]
pub struct Response {
    data: Vec<Space>,
    includes: Option<Includes>,
}

#[derive(Clone, Debug)]
pub enum RateLimitContext {
    Application(String),
    AccessToken(RequestCredential),
}

impl Response {
    pub fn spaces(&self) -> &Vec<Space> {
        self.data.as_ref()
    }
}

impl ResponseTrait for Response {
    type Response = Response;

    fn try_into_from_bytes(bytes: &[u8]) -> Result<Response, XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}
