use super::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Response {
    data: Vec<Space>,
    includes: Option<Includes>,
}

impl Response {
    pub fn spaces(&self) -> &Vec<Space> {
        self.data.as_ref()
    }
}

impl<'a> super::Response for Response {
    type Response = Response;

    fn try_into_from_bytes(bytes: &[u8]) -> Result<Response, XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}
