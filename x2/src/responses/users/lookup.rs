use serde::Deserialize;

use crate::model::error::XError;
use crate::model::users::{Includes, Users};

pub use crate::responses::Response as ResponseTrait;

#[derive(Debug, Deserialize)]
pub struct Response {
    data: Option<Users>,
    includes: Option<Includes>,
}

impl<'a> Response {
    pub fn users(&self) -> &Option<Users> {
        &self.data
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
