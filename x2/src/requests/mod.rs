use std::{fmt::Display, sync::OnceLock};
use strum::AsRefStr;

pub use reqwest;

use crate::model::error::XError;

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub mod auth;
//pub mod rate_limit;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;

pub trait Request<'a> {
    type Response;

    fn authorize(&mut self) -> Result<(), XError>;
    fn is_authorized(&self) -> bool;
    fn request(&self) -> Result<Self::Response, XError>;
}

static BASE_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

pub fn client() -> &'static reqwest::blocking::Client {
    BASE_CLIENT.get_or_init(|| {
        reqwest::blocking::ClientBuilder::new()
            .http2_prior_knowledge()
            .user_agent(APP_USER_AGENT)
            .referer(false)
            .https_only(true)
            .gzip(true)
            .build()
            .unwrap()
    })
}
