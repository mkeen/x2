use super::prelude::*;

use crate::requests::{auth::Request as AuthRequest, Request};

use strum::EnumIs;
use urlencoding::encode;

#[derive(EnumIs, Debug, Clone)]
pub enum Context<'a> {
    Caller(Method<'a>),
    Request(RequestCredential),
}

impl<'a> Context<'a> {
    pub fn authenticate(&self) -> Result<Self, XError> {
        match self {
            Context::Request(_) => Ok(self.clone()),
            Context::Caller(_) => AuthRequest::new(&self).request().map(|r| r.into()),
        }
    }

    pub fn authorize(
        &self,
        builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        match self {
            Context::Caller(unauthenticated) => match *unauthenticated {
                Method::AppOnly { id, secret } => {
                    builder.basic_auth(encode(id), Some(encode(secret)))
                }
            },
            Context::Request(authenticated) => match authenticated {
                RequestCredential::Bearer(bearer) => builder.bearer_auth(bearer),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Method<'a> {
    AppOnly { id: &'a str, secret: &'a str },
}

#[derive(Debug, EnumIs, Clone)]
pub enum RequestCredential {
    Bearer(String),
}
