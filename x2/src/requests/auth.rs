use std::collections::HashMap;

use super::prelude::*;

pub use crate::responses::auth::Response as AuthResponse;

use super::Authorized;

static PARAMS: [(&str, &str); 1] = [("grant_type", "client_credentials")];

pub struct Request {
    builder: super::RequestBuilder,
}

impl<'a> Request {
    pub fn new(auth: &'a Context) -> Self {
        Self {
            builder: Self::builder_with_auth(
                auth,
                super::client()
                    .post(Endpoint::Authentication.url())
                    .form(&HashMap::from(PARAMS)),
            ),
        }
    }
}

impl Authorized<AuthResponse> for Request {}

impl<'a> super::Request<AuthResponse> for Request {
    fn request(self) -> Result<AuthResponse, XError> {
        self.builder
            .send()
            .map_err(|e| XError::Socket(e.to_string()))
            .map(|response| match response.status().is_success() {
                true => AuthResponse::try_into_from_bytes(
                    &response.bytes().map_err(|e| XError::Reqwest(e))?,
                ),

                false => Err(XError::HttpGeneric(
                    response.status(),
                    response.text().unwrap_or("Unknown".into()),
                )),
            })?
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::auth::Method, requests::Request};

    use super::{Request as AuthRequest, *};

    #[test]
    // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
    fn auth_request_app_only_to_bearer<'a>() {
        let id = "GJe6IFjFNwveQzBhJmaMIZzW5";
        let secret = "f9kmkg3eQkxNB7thibc5lhhgavCq4eQmMrTdeO9aw4rIz4Hofb";

        let response = AuthRequest::new(&Context::Caller(Method::AppOnly { id, secret })).request();

        assert!(response.is_ok());
        assert!(response.unwrap().is_bearer());
    }
}
