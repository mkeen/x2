use std::sync::OnceLock;

use arrayvec::ArrayVec;
pub use reqwest;

use crate::model::{
    auth::Authorized,
    error::{XAuthError, XError},
};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub mod auth;
//pub mod rate_limit;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;

type RequestBuilder = reqwest::blocking::RequestBuilder;

pub trait Request<T>
where
    T: crate::responses::Response,
{
    fn request(self) -> Result<T, XError>;
}

pub trait AuthorizedRequest<T>: Request<T> + Authorized<T>
where
    T: crate::responses::Response,
{
}

static BASE_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

// todo: rename to make it clear this returns a csv
pub fn collect_csv<T, const N: usize>(list: &[T]) -> String
where
    T: AsRef<str>,
{
    match list.is_empty() {
        true => "".to_string(),
        false => list
            .iter()
            .map(|e| e.as_ref())
            .collect::<ArrayVec<&str, N>>()
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
