use std::{fmt::Display, sync::OnceLock};

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

    fn request(&self) -> Result<Self::Response, XError>;
}

pub fn fields_as_csv<T>(fields: &Vec<T>) -> String
where
    T: Display,
{
    fields
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn push_to_params(target: &mut Vec<(String, String)>, param: &String, name: &str) {
    if !param.is_empty() {
        target.push((name.into(), param.into()))
    }
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
