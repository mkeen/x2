use std::collections::HashMap;

use super::prelude::*;

pub use crate::responses::auth::Response;

static PARAMS: [(&str, &str); 1] = [("grant_type", "client_credentials")];

pub struct Request {
    builder: Option<super::RequestBuilder>,
}

impl<'a> Request {
    pub fn new(auth: &'_ Context) -> Self {
        Self {
            builder: Self::builder_with_auth(
                auth,
                super::client()
                    .post(Endpoint::Authentication.url(None))
                    .form(&HashMap::from(PARAMS)),
            ),
        }
    }
}

impl Authorized<Response> for Request {}

impl super::Request<Response> for Request {
    fn builder(&mut self) -> Option<super::RequestBuilder> {
        self.builder.take()
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::auth::Method, requests::Request};

    use super::{Request as AuthRequest, *};

    #[test]
    // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
    fn auth_request_app_only_to_bearer<'a>() {
        let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
        let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

        let response = AuthRequest::new(&Context::Caller(Method::AppOnly { id, secret })).request();

        assert!(response.is_ok());
        assert!(response.unwrap().is_bearer());
    }
}
