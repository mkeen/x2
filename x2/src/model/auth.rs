use super::prelude::*;

use crate::requests::{auth::Request as AuthRequest, Request};

use urlencoding::encode;

use reqwest_oauth1::OAuthClientProvider;

#[derive(EnumIs, Debug, Clone)]
pub enum Context {
    Caller(Method),
    Request(RequestCredential),
}

impl Context {
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
            Context::Caller(unauthenticated) => match unauthenticated {
                Method::AppOnly { id, secret } => {
                    builder.basic_auth(encode(&id), Some(encode(&secret)))
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
            Context::Caller(unauthenticated) => panic!("dah!"),
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

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(name(MethodType))]
pub enum Method {
    AppOnly { id: String, secret: String },
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
