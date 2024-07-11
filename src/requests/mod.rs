pub mod auth;

use crate::model::{auth::RequestCredential, error::XError};

pub trait Request {
    fn request(self) -> Result<RequestCredential, XError>;
}
