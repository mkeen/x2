use super::prelude::*;

use std::collections::HashMap;

use responses::auth::Response;

use model::auth::Method;

static PARAMS: [(&str, &str); 1] = [("grant_type", "client_credentials")];

#[derive(Debug, Built, Authorized)]
pub struct Request {
    builder: Option<RequestBuilder>,
}

impl Request {
    pub fn new(auth: &Context) -> Self {
        match auth {
            Context::Caller(caller) => match caller {
                Method::OAuth10AUser {
                    app_id,
                    app_secret,
                    user_id,
                    user_secret,
                } => Self {
                    builder: Some(RequestBuilder::Oauth1(
                        Self::authorize_oauth1(auth).get(Endpoint::Authentication10A.url(None)),
                    )),
                },

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
    use crate::model::auth::Method;

    #[test]
    fn auth_request_app_only_to_bearer<'a>() {
        let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
        let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

        let context = Context::Caller(Method::AppOnly { id, secret });
        let response = Request::new(&context).request();

        assert!(response.is_ok());
        assert!(response.unwrap().is_bearer());
    }
}
