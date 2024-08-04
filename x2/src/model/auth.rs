use super::prelude::*;

use crate::requests::{auth::Request as AuthRequest, Request};

use reqwest::{
    blocking::Client,
    header::{HeaderName, HeaderValue},
};
use strum::EnumIs;
use urlencoding::encode;

use reqwest;
use reqwest_oauth1::OAuthClientProvider;

#[derive(EnumIs, Debug, Clone)]
pub enum Context<'a> {
    Caller(Method<'a>),
    Request(RequestCredential),
}

pub enum BuildPhase {
    Client(Client),
    Builder(reqwest::blocking::RequestBuilder),
}

impl<'a> Context<'a> {
    pub fn authenticate(&self) -> Result<Self, XError> {
        match self {
            Context::Request(_) => Ok(self.clone()),
            Context::Caller(_) => AuthRequest::new(&self).request().map(|r| r.into()),
        }
    }

    pub fn authorize_simple(
        &self,
        builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        match self {
            Context::Caller(unauthenticated) => match *unauthenticated {
                Method::AppOnly { id, secret } => {
                    builder.basic_auth(encode(id), Some(encode(secret)))
                }
                _ => builder,
            },
            Context::Request(authenticated) => match authenticated {
                RequestCredential::Bearer(bearer) => builder.bearer_auth(bearer),
                _ => builder,
            },
        }
    }

    pub fn authorize_oauth1(
        &self,
        client: &'static reqwest::blocking::Client,
    ) -> reqwest_oauth1::Client<
        reqwest_oauth1::Signer<reqwest_oauth1::Secrets, reqwest_oauth1::DefaultSM>,
    > {
        match self {
            Context::Caller(unauthenticated) => match *unauthenticated {
                Method::OAuth10AUser {
                    app_id,
                    app_secret,
                    user_id,
                    user_secret,
                } => client.clone().oauth1(
                    reqwest_oauth1::Secrets::new(app_id, app_secret).token(user_id, user_secret),
                ),
                _ => panic!("dah!"),
            },
            Context::Request(authenticated) => match authenticated {
                RequestCredential::Bearer(bearer) => panic!("dah!"),
                RequestCredential::OAuth10AConsumer {
                    consumer_id,
                    consumer_secret,
                    user_id,
                    user_secret,
                } => client.clone().oauth1(
                    reqwest_oauth1::Secrets::new(consumer_id, consumer_secret)
                        .token(user_id, user_secret),
                ),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Method<'a> {
    AppOnly {
        id: &'a str,
        secret: &'a str,
    },
    OAuth10AUser {
        app_id: &'a str,
        app_secret: &'a str,
        user_id: &'a str,
        user_secret: &'a str,
    },
}

#[derive(Debug, EnumIs, Clone)]
pub enum RequestCredential {
    Bearer(String),
    OAuth10AConsumer {
        consumer_id: String,
        consumer_secret: String,
        user_id: String,
        user_secret: String,
    },
}
