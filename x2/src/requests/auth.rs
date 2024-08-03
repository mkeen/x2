use super::prelude::*;

use std::collections::HashMap;

use responses::auth::Response;

static PARAMS: [(&str, &str); 1] = [("grant_type", "client_credentials")];

#[derive(Debug, Built, Authorized)]
pub struct Request {
    builder: Option<super::RequestBuilder>,
}

impl Request {
    pub fn new(auth: &Context) -> Self {
        Self {
            builder: Self::authorize(
                auth,
                super::client()
                    .post(Endpoint::Authentication.url(None))
                    .form(&HashMap::from(PARAMS)),
            ),
        }
    }
}

#[derive(Debug, EnumProperty)]
pub enum Endpoint {
    #[strum(props(Path = "oauth2/token"))]
    Authentication,
}

impl super::Endpoint for Endpoint {}

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
