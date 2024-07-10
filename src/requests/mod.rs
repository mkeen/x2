pub mod auth;

use crate::{model::error::XError, responses::Response};

pub trait Request {
    fn request(&self) -> Result<Response, XError>;
}
