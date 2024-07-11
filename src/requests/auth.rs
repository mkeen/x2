use oauth2::{basic::BasicClient, reqwest::http_client, AuthUrl, TokenResponse, TokenUrl};

use crate::{
    config::{endpoint_url, EndpointId},
    model::{
        auth::{AppCredential, RequestCredential},
        error::XError,
    },
};

pub struct Request {
    url: reqwest::Url,
    credential: AppCredential,
}

impl Request {
    pub fn new(credential: AppCredential) -> Self {
        Request {
            credential,
            url: endpoint_url(EndpointId::Authentication)
                .expect("lib error, could not find url for request type"),
        }
    }

    pub fn client(self) -> BasicClient {
        match self.credential {
            AppCredential::AppOnly {
                client_id,
                client_secret,
            } => BasicClient::new(
                client_id,
                Some(client_secret),
                AuthUrl::new("https://localhost/stub".to_string()).expect("sorry"),
                Some(TokenUrl::from_url(self.url)),
            ),
        }
    }
}

impl super::Request for Request {
    fn request(self) -> Result<RequestCredential, XError> {
        self.client()
            .exchange_client_credentials()
            .request(http_client)
            .map_err(|e| XError::Auth(e.to_string()))
            .map(|r| RequestCredential::Bearer(r.access_token().to_owned()))
    }
}
