pub(crate) mod prelude {
    pub use super::super::_prelude::*;
    pub use super::util::csv;
    pub use crate::config::Endpoint;
    pub use crate::model::auth::{Authorized, Context};
    pub use crate::responses::Response;
    pub use reqwest::Url;
    pub use serde::Serialize;

    pub static DEFAULT_RESULT_LIMIT: u16 = 25;
}

use bytes::Bytes;
use prelude::*;

use std::{collections::HashSet, sync::OnceLock, usize};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static BASE_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

pub mod auth;
//pub mod limits;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;
pub mod util;

pub type RequestBuilder = reqwest::blocking::RequestBuilder;

pub trait Request<T: Response> {
    fn builder(&mut self) -> Option<RequestBuilder>;

    fn request(&mut self) -> Result<T, XError>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.builder()
            .map(|builder| {
                builder
                    .send()
                    .map_err(|e| XError::Socket(e.to_string()))
                    .map(|response| match response.status().is_success() {
                        true => T::try_into_from_bytes(
                            &response.bytes().map_err(|e| XError::Reqwest(e))?,
                        ),
                        false => Err(XError::HttpGeneric(
                            response.status(),
                            response.text().unwrap_or("Unknown".into()),
                        )),
                    })?
            })
            .ok_or(XError::AlreadyConsumed)?
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
            .build()
            .expect("failed to initialize the http client")
    })
}
