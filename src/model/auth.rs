use super::error::XError;
use crate::{requests::Request, responses::Response};
use oauth2::AccessToken;
use serde::Serialize;
use strum::EnumDiscriminants;

#[derive(Debug, Clone)]
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
impl<'a> TryInto<RequestCredential> for AppCredential<'a> {
    type Error = XError;

    fn try_into(self) -> Result<RequestCredential, XError> {
        crate::requests::auth::Request::new(self)
            .request()
            .map(|a| match a {
                crate::responses::auth::Response::Bearer(bearer) => {
                    RequestCredential::Bearer(bearer)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
    fn create_basic_request_credential() {
        let client_id = "gUJTmN2jcD7zOg2kFcbbS3fSp";
        let client_secret = "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon";

        let authentication: Result<RequestCredential, XError> = AppCredential::AppOnly {
            client_id,
            client_secret,
        }
        .try_into();

        println!("{:?}", authentication);

        assert!(authentication.is_ok())
    }
}
