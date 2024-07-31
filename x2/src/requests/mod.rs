pub(crate) mod prelude {
    pub use super::super::_prelude::*;
    pub use crate::config::Endpoint;
    pub use crate::model::auth::{Authorized, Context};
    pub use crate::responses::Response;
}

use prelude::*;

use arrayvec::ArrayVec;
use std::{fmt::Display, sync::OnceLock};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static BASE_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

pub mod auth;
//pub mod limits;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;

pub type RequestBuilder = reqwest::blocking::RequestBuilder;

pub trait Request<T: Response> {
    fn request(self) -> Result<T, XError>;
}

// todo: rename to make it clear this returns a csv
pub fn collect_csv<T, const N: usize>(list: &[T]) -> String
where
    T: AsRef<str> + EnumCount,
{
    match list.is_empty() {
        true => "".to_string(),
        false => list
            .iter()
            .map(|e| e.as_ref())
            .collect::<ArrayVec<&str, N>>() // todo: benchmark against Vec
            .join(","),
    }
}

pub fn client() -> &'static reqwest::blocking::Client {
    BASE_CLIENT.get_or_init(|| {
        reqwest::blocking::ClientBuilder::new()
            .http2_prior_knowledge() // todo: remove, but look into other optimizations
            .user_agent(APP_USER_AGENT)
            .referer(false)
            .https_only(true)
            .gzip(true)
            .http1_title_case_headers()
            .build()
            .expect("failed to initialize the http client")
    })
}
