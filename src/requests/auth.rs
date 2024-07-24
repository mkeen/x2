use std::collections::HashMap;

use urlencoding::encode;

use crate::{
    config::Endpoint,
    model::{
        auth::{AppCredential, Credential},
        error::XError,
    },
    responses::auth::*,
};

pub struct Request<'a> {
    client: &'a reqwest::blocking::Client,
    credential: &'a Credential<'a>,
}

pub enum Expansion {}

pub enum Params {}

pub enum ParamName {}

pub struct Fields {}

pub struct Options {
    params: Option<Params>,
    expansions: Option<Vec<Expansion>>,
    fields: Option<Fields>,
}

impl<'a> Request<'a> {
    pub fn new(credential: &'a Credential, _: Option<Options>) -> Self {
        Self {
            client: super::client(),
            credential,
        }
    }
}

impl<'a> super::Request<'a> for Request<'a> {
    type Response = Response;

    #[allow(refining_impl_trait)]
    fn request(&self) -> Result<Response, XError> {
        match self.credential {
            Credential::Unauthorized(AppCredential::AppOnly {
                client_id,
                client_secret,
            }) => {
                let client_secret = encode(client_secret);
                let client_id = encode(client_id);

                let mut params = HashMap::new();
                params.insert("grant_type", "client_credentials");

                let bytes = self
                    .client
                    .post(Endpoint::Authentication.url())
                    .basic_auth(client_id, Some(client_secret))
                    .form(&params)
                    .send()
                    .map_err(|e| crate::model::error::XError::Auth(e))?
                    .bytes()
                    .map_err(|e| crate::model::error::XError::Auth(e))?;

                Response::try_into_from_bytes(&bytes.to_vec())
            }
            Credential::Authorized(c) => match c {
                crate::model::auth::RequestCredential::Bearer(bearer) => {
                    Ok(Response::Bearer(bearer.into()))
                }
            },
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
//     fn create_basic_request_credential<'a>() {
//         let client_id = "gUJTmN2jcD7zOg2kFcbbS3fSp";
//         let client_secret = "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon";

//         let authentication: Result<RequestCredential<'a>, XError> =
//             Credential::AppCredential(AppCredential::AppOnly {
//                 client_id,
//                 client_secret,
//             })
//             .try_into();

//         println!("{:?}", authentication);

//         assert!(authentication.is_ok())
//     }
// }
