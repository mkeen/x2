use super::error::XError;
use crate::requests::Request;

use serde::Serialize;
use strum::EnumDiscriminants;

pub enum Credential {
    Unauthorized(AppCredential),
    Authorized(RequestCredential),
}

#[derive(Debug)]
pub enum AppCredential {
    AppOnly {
        client_id: String,
        client_secret: String,
    },
}

#[derive(Debug, Clone, EnumDiscriminants, Serialize)] // todo impl eq
#[strum_discriminants(name(CredentialType))]
pub enum RequestCredential {
    Bearer(String),
}

impl<'a> TryInto<RequestCredential> for Credential {
    type Error = XError;

    fn try_into(self) -> Result<RequestCredential, XError> {
        match self {
            Credential::Unauthorized(app_credential) => {
                crate::requests::auth::Request::new(&self, None)
                    .request()
                    .map(|a| match a {
                        crate::responses::auth::Response::Bearer(bearer) => {
                            RequestCredential::Bearer(bearer)
                        }
                    })
            }
            Credential::Authorized(request_credential) => Ok(request_credential),
        }
    }
}

impl<'a> TryInto<&'a RequestCredential> for &Credential {
    type Error = XError;

    fn try_into(self) -> Result<&'a RequestCredential, XError> {
        match self {
            Credential::Unauthorized(app_credential) => {
                crate::requests::auth::Request::new(&self, None)
                    .request()
                    .map(|a| match a {
                        crate::responses::auth::Response::Bearer(bearer) => {
                            &RequestCredential::Bearer(bearer)
                        }
                    })
            }
            Credential::Authorized(request_credential) => Ok(request_credential),
        }
    }
}
