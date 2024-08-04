pub(crate) mod prelude {
    pub(crate) type DefaultSigner = Signer<reqwest_oauth1::Secrets, reqwest_oauth1::DefaultSM>;
    pub use super::super::_prelude::*;
    pub use super::super::model::auth::Context;
    pub use super::super::responses::Response;
    pub use super::util::csv;
    pub(crate) use super::Endpoint as EndpointTrait;
    use reqwest_oauth1::Signer;
    pub use serde::Serialize;
    pub static DEFAULT_RESULT_LIMIT: u16 = 25;
    pub(crate) use super::super::responses;
    pub use super::Authorized as AuthorizeTrait;
    pub use super::Request as RequestTrait;
    pub(crate) use x2_derive::{Authorized, Built, UrlEndpoint};
    pub(crate) type RequestBuilder = super::ClientAgnosticBuilder;
    pub(crate) type Oauth1RequestBuilder = reqwest_oauth1::RequestBuilder<DefaultSigner>;
}

use prelude::*;
use reqwest::{blocking::Client as ReqwestClient, Url};

use std::sync::OnceLock;

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static BASE_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

pub mod auth;
//pub mod limits;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;
pub mod util;

pub trait Request<R: Response>: Sized {
    fn builder(&mut self) -> Option<ClientAgnosticBuilder>;
    fn update_builder(&mut self, builder: ClientAgnosticBuilder);

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

#[derive(Debug)]
pub enum ClientAgnosticBuilder {
    Native(reqwest::blocking::RequestBuilder),
    Oauth1(Oauth1RequestBuilder), // todo: check upstream for why a lifetime is needed here
}

impl ClientAgnosticBuilder {
    fn get_oauth1(self) -> Option<Oauth1RequestBuilder> {
        match self {
            ClientAgnosticBuilder::Oauth1(oauth1) => Some(oauth1),
            _ => None,
        }
    }

    fn get_native(self) -> Option<reqwest::blocking::RequestBuilder> {
        match self {
            ClientAgnosticBuilder::Native(native) => Some(native),
            _ => None,
        }
    }

    fn send(self) -> Result<reqwest::blocking::Response, XError> {
        match self {
            ClientAgnosticBuilder::Native(native) => native.send().map_err(|e| XError::Reqwest(e)),
            ClientAgnosticBuilder::Oauth1(oauth1) => {
                oauth1.send().map_err(|e| XError::ReqwestOauth1(e))
            }
        }
    }
}

pub(crate) fn client() -> &'static ReqwestClient {
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
    fn authorize_simple(
        auth: &Context,
        builder: reqwest::blocking::RequestBuilder,
    ) -> Option<RequestBuilder> {
        Some(RequestBuilder::Native(auth.authorize_simple(builder)))
    }

    fn authorize_oauth1(
        auth: &Context,
    ) -> reqwest_oauth1::Client<
        reqwest_oauth1::Signer<reqwest_oauth1::Secrets, reqwest_oauth1::DefaultSM>,
    > {
        auth.authorize_oauth1(client())
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
