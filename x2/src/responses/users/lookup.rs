pub use derive_getters::Getters;
use serde::Deserialize;

use crate::model::error::XError;
use crate::model::users::{Includes, User};

pub use crate::responses::Response as ResponseTrait;

#[derive(Debug, Deserialize, Getters)]
pub struct Response {
    #[getter(rename = "users")]
    data: Vec<User>,
    includes: Option<Includes>,
}

impl ResponseTrait for Response {
    type Response = Response;

    fn try_into_from_bytes(bytes: &[u8]) -> Result<Response, XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}
