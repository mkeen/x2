use std::collections::HashMap;

use crate::{
    model::{
        auth::{Credential, RequestCredential},
        error::XError,
    },
    Expansion,
};

use super::{Field, Param, ParamName};

use crate::responses::rate_limit::*;

pub struct Request<'a> {
    client: &'a reqwest::blocking::Client,
    credential: RequestCredential,
}

impl<'a> super::Request<'a> for Request<'a> {
    fn new(
        credential: Credential,
        _: Option<&'a HashMap<impl ParamName, impl Param>>,
        _: Option<&'a Vec<impl Expansion>>,
        _: Option<&'a Vec<impl Field>>,
    ) -> Self {
        Self {
            client: super::client(),
            credential: credential.try_into().expect("API Credentials Required"),
        }
    }

    #[allow(refining_impl_trait)]
    fn request(
        self,
    ) -> Result<crate::responses::rate_limit::Response, crate::model::error::XError> {
        match self.credential {
            RequestCredential::Bearer(bearer) => Response::try_from_bytes(
                &self
                    .client
                    .get(crate::config::Endpoint::RateLimit.url())
                    .bearer_auth(bearer)
                    .send()
                    .map_err(|e| XError::Auth(e))?
                    .bytes()
                    .unwrap(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            auth::{AppCredential, RequestCredential},
            error::XError,
        },
        requests::Request,
    };

    #[test]
    // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
    fn do_rate_limit_() {
        let client_id = "gUJTmN2jcD7zOg2kFcbbS3fSp";
        let client_secret = "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon";

        let authentication: Result<RequestCredential, XError> = AppCredential::AppOnly {
            client_id,
            client_secret,
        }
        .try_into();

        if let Ok(auth) = authentication {
            assert!(crate::requests::rate_limit::Request::new(auth)
                .request()
                .is_ok())
        } else {
            assert!(false);
        }
    }
}
