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

pub trait Response {
    fn try_into_from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T, crate::model::error::XError>
    where
        T: Deserialize<'a>,
    {
        serde_json::from_slice::<T>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Meta {
    pub result: Option<u64>,
}

#[derive(Debug, Deserialize, Getters)]
pub struct Data<D, I> {
    pub data: D,
    pub includes: Option<I>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Getters)]
pub struct SimpleData<D> {
    pub data: D,
}
