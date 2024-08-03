pub mod prelude {
    pub use super::super::_prelude::*;
    pub use super::super::requests::Request;
    pub use super::Pattern;
}

use prelude::*;

pub mod auth;
//pub mod rate_limit;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;

pub type Pattern<T> = T;

pub trait Response: for<'de> Deserialize<'de> {
    fn try_into_from_bytes<'de>(bytes: &'de [u8]) -> Result<Self, crate::model::error::XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Meta {
    pub result: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Data<D, I> {
    pub data: D,
    pub includes: Option<I>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleData<D> {
    pub data: D,
}
