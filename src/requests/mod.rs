use lazy_static::lazy_static;
use reqwest::blocking::ClientBuilder;

use crate::{model::error::XError, responses::Response};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

lazy_static! {
    pub static ref BASE_CLIENT: ClientBuilder = {
        ClientBuilder::new()
            .http2_prior_knowledge()
            .user_agent(APP_USER_AGENT)
            .referer(false)
            .https_only(true)
            .gzip(true)
    };
}

pub mod auth;
pub mod rate_limit;

pub trait Request<'a> {
    fn request(&self) -> Result<impl Response<'a>, XError>;
}
