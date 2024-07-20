use crate::model::{auth::RequestCredential, error::XError};

pub struct Request {
    client: reqwest::blocking::Client,
    credential: RequestCredential,
}

impl Request {
    pub fn new(credential: RequestCredential) -> Self {
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

impl<'a> super::Request<'a> for Request {
    #[allow(refining_impl_trait)]
    fn request(
        &self,
    ) -> Result<crate::responses::rate_limit::Response, crate::model::error::XError> {
        match &(self.credential) {
            RequestCredential::Bearer(bearer) => {
                let r = self
                    .client
                    .get(crate::config::Endpoint::RateLimit.url())
                    .bearer_auth(bearer)
                    .send();

                crate::responses::rate_limit::Response::try_from_bytes(
                    &r.map_err(|e| XError::Auth(e))?.bytes().unwrap(),
                )
            }
        }
    }
}
