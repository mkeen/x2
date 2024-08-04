pub mod prelude {
    pub use super::super::_prelude::*;
    pub use super::super::requests;
    pub use super::Pattern;
    pub(crate) use requests::{Endpoint, Request};
}

use std::marker::PhantomData;

use prelude::*;

pub mod auth;
//pub mod rate_limit;
pub mod spaces;
//pub mod usage_tweets;
pub mod tweets;
pub mod users;

pub type Pattern<T> = T;

pub trait Response: for<'de> Deserialize<'de> {
    type Request: super::requests::Request<Self>;

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
pub struct Data<D, I, const ID: usize> {
    pub data: D,
    pub includes: Option<I>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleData<D> {
    pub data: D,
}
