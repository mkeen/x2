use oauth2::{reqwest::http_client, TokenResponse};

use crate::{
    config::{endpoint_url, EndpointId},
    model::{
        auth::{AppCredential, Authentication, RequestCredential},
        error::XError,
    },
    responses::Response,
};

pub struct Request<'a> {
    url: reqwest::Url,
    app_credential: &'a AppCredential,
}

impl<'a> Request<'a> {
    pub fn new(app_credential: &'a AppCredential) -> Self {
        Request {
            app_credential,
            url: endpoint_url(EndpointId::Authentication)
                .expect("lib error, could not find url for request type"),
        }
    }
}

impl<'a> super::Request for Request<'a> {
    fn request(&self) -> Result<Response, XError> {
        Authentication::GetBearerToken(&self.app_credential)
            .client(
                oauth2::AuthUrl::from_url(self.url.clone()),
                Some(oauth2::TokenUrl::from_url(self.url.clone())),
            )
            .exchange_client_credentials()
            .request(http_client)
            .map_err(|e| XError::Auth(e.to_string()))
            .map(|token_response| {
                Response::AuthToken(RequestCredential::Bearer(
                    token_response.access_token().to_owned(), // todo error is because some trait needs to be imported
                ))
            })
    }
}
