use std::collections::HashMap;

use urlencoding::encode;

use crate::{
    config::Endpoint,
    model::{auth::AppCredential, error::XError},
};

pub struct Request<'a> {
    client: reqwest::blocking::Client,
    credential: AppCredential<'a>,
}

impl<'a> Request<'a> {
    pub fn new(credential: AppCredential<'a>) -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .http2_prior_knowledge()
                .user_agent(super::APP_USER_AGENT)
                .referer(false)
                .https_only(true)
                .gzip(true)
                .build()
                .unwrap(),
            credential,
        }
    }
}

impl<'a> super::Request<'a> for Request<'a> {
    #[allow(refining_impl_trait)]
    fn request(&self) -> Result<crate::responses::auth::Response, XError> {
        match self.credential {
            AppCredential::AppOnly {
                client_id,
                client_secret,
            } => {
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

                crate::responses::auth::Response::try_from_bytes(bytes.to_vec())
            }
        }
    }
}
