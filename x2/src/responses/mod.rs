pub mod prelude {
    pub use super::super::_prelude::*;
}

use prelude::*;

pub mod auth;
pub mod rate_limit;
pub mod spaces;
pub mod usage_tweets;
pub mod users;

pub trait Response {
    type Response;

    fn try_into_from_bytes(bytes: &[u8]) -> Result<Self::Response, crate::model::error::XError>;
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Meta {
    result: Option<u64>,
}
