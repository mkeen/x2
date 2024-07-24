use super::error::XError;
use crate::requests::Request;

use serde::Serialize;
use strum::EnumDiscriminants;

pub enum Credential<'a> {
    Unauthorized(AppCredential<'a>),
    Authorized(RequestCredential),
}

#[derive(Debug)]
pub enum AppCredential<'a> {
    AppOnly {
        client_id: &'a str,
        client_secret: &'a str,
    },
}

#[derive(Debug, Clone, EnumDiscriminants, Serialize)] // todo impl eq
#[strum_discriminants(name(CredentialType))]
pub enum RequestCredential {
    Bearer(String),
}

#[allow(refining_impl_trait)]
impl<'a> TryInto<RequestCredential> for &'a Credential<'a> {
    type Error = XError;

    fn try_into(self) -> Result<RequestCredential, XError> {
        match self {
            Credential::Unauthorized(app_credential) => {
                crate::requests::auth::Request::new(self, None)
                    .request()
                    .map(|a| match a {
                        crate::responses::auth::Response::Bearer(bearer) => {
                            RequestCredential::Bearer(bearer.into())
                        }
                    })
            }
            Credential::Authorized(request_credential) => Ok(request_credential.to_owned()),
        }
    }
}
