pub(crate) mod prelude {
    pub use super::super::_prelude::*;
    pub use super::super::model::auth::Context;
    pub use super::super::responses::Response;
    pub use super::util::csv;
    pub(crate) use super::Endpoint as EndpointTrait;
    pub use serde::Serialize;
    pub static DEFAULT_RESULT_LIMIT: u16 = 25;
    pub(crate) use super::super::responses;
    pub use super::Authorized as AuthorizeTrait;
    pub use super::Request as RequestTrait;
    pub(crate) use x2_derive::{Authorized, Built};
    pub(crate) type RequestBuilder = reqwest::blocking::RequestBuilder;
}

use bytes::Bytes;
use prelude::*;
use reqwest::Url;

use std::sync::OnceLock;

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static BASE_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

pub mod auth;
//pub mod limits;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;
pub mod util;

pub trait Request<R: Response> {
    fn builder(&mut self) -> Option<RequestBuilder>;
    fn update_builder(&mut self, builder: reqwest::blocking::RequestBuilder);

    fn request(&mut self) -> Result<R, XError> {
        self.builder()
            .map(|builder| {
                builder
                    .send()
                    .map_err(|e| XError::Socket(e.to_string()))
                    .map(|response| match response.status().is_success() {
                        true => R::try_into_from_bytes(
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

pub(crate) fn client() -> &'static reqwest::blocking::Client {
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

pub trait Authorized<R>: Request<R>
where
    R: Response,
{
    fn authorize(auth: &Context, builder: RequestBuilder) -> Option<RequestBuilder> {
        Some(auth.authorize(builder))
    }
}

pub(crate) trait Endpoint: EnumProperty {
    fn get_request_path(&self) -> &str {
        self.get_str(super::config::URL_PARAM_NAME).unwrap()
    }

    fn replace_url_params(&self, params: &[&str]) -> String {
        self.get_request_path()
            .split(super::config::URL_PARAM_FLAG)
            .enumerate()
            .map(|(i, s)| format!("{}{}", s, params.get(i).map_or("", |&x| x)))
            .collect::<String>()
    }

    fn url(&self, params: Option<&[&str]>) -> Url {
        let params = params.unwrap_or(&super::config::DEFAULT_URL_PARAMS);
        Url::parse(&format!(
            "https://api.twitter.com/{}",
            self.replace_url_params(params)
        ))
        .expect("bad static url definition or params implementation")
    }
}
