pub(crate) mod prelude {
    pub use super::super::_prelude::*;
    pub use super::model::responses::{Data, Meta, SimpleData};
}

use prelude::*;

pub mod auth;
//pub mod rate_limit;
//pub mod spaces;
//pub mod usage_tweets;
pub mod tweets;
//pub mod users;

pub trait Response<'a>: for<'de> Deserialize<'de> {
    type Request: super::requests::Request<'a, Self>;

    fn try_into_from_bytes<'de>(bytes: &'de [u8]) -> Result<Self, XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}

pub trait Paginated<'a>: Response<'a> {
    fn next_page(&self) -> Option<&String>;
    fn meta(&self) -> &Option<Meta>;
}
