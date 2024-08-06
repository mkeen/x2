use super::prelude::*;

use std::collections::HashMap;

use responses::auth::Response;

use model::auth::Method;

static PARAMS: [(&str, &str); 1] = [("grant_type", "client_credentials")];

#[derive(Debug, Built, Authorized)]
pub struct Request<'a> {
    builder: Option<RequestBuilder<'a>>,
}

impl<'a> Request<'a> {
    pub fn new(auth: &'a Context) -> Self {
        match auth {
            Context::Caller(caller) => match caller {
                Method::AppOnly { id, secret } => {
                    // todo, not the cleanest that we have id and secret in scope here
                    Self {
                        builder: Self::authorize_simple(
                            auth,
                            super::client()
                                .post(Endpoint::Authentication.url(None))
                                .form(&HashMap::from(PARAMS)),
                        ),
                    }
                }
            },

            _ => panic!("wrong auth creds"),
        }
    }
}

#[derive(Debug, EnumProperty, UrlEndpoint)]
pub enum Endpoint {
    #[strum(props(Path = "oauth2/token"))]
    Authentication,
    #[strum(props(Path = "oauth/request_token"))]
    Authentication10A,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{model::auth::Method, test_util::app_only_unauthed_credentials};

    #[test]
    fn auth_request_app_only_to_bearer<'a>() {
        let context = app_only_unauthed_credentials();

        let response = Request::new(&context).request();

        assert!(response.is_ok());
        assert!(response.unwrap().is_bearer());
    }
}
