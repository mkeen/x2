use super::prelude::*;

use crate::{
    requests::{auth::Request as AuthRequest, Request},
    responses::{self, Response},
};

use strum::EnumIs;
use urlencoding::encode;

#[derive(EnumIs, Debug)]
pub enum Context<'a> {
    Caller(Method<'a>),
    Request(RequestCredential),
}

impl<'a> Context<'a> {
    pub fn is_authorized(&self) -> bool {
        self.is_request()
    }

    pub fn authorize(self) -> Result<Self, XAuthError> {
        AuthRequest::new(&self)
            .request()
            .map_err(|e| XAuthError::Upstream(e.to_string()))
            .map(|c| match c {
                responses::auth::Response::Bearer(bearer) => {
                    Context::Request(RequestCredential::Bearer(bearer))
                }
            })
    }
}

#[derive(Debug)]
pub enum Method<'a> {
    AppOnly { id: &'a str, secret: &'a str },
}

#[derive(Debug, EnumIs)]
pub enum RequestCredential {
    Bearer(String),
}

// todo: move this to request since its more relevant to requests
pub trait Authorized<T>: Request<T>
where
    T: Response,
{
    fn builder_with_auth(
        auth: &Context,
        builder: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        match auth {
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
