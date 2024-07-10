use crate::requests::Request;
use crate::responses::Response;

use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    AuthUrl, Client, EmptyExtraTokenFields, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};

use super::error::XError;

#[derive(Clone)]
pub enum RequestCredential {
    Bearer(oauth2::AccessToken),
}

#[derive(Clone)]
pub enum AppCredential {
    AppOnly {
        client_id: oauth2::ClientId,
        client_secret: oauth2::ClientSecret,
    },
}

impl AppCredential {
    pub fn to_request_credential(&self) -> Result<RequestCredential, XError> {
        self.authenticate().map(|r| match r {
            Response::AuthToken(creds) => creds,
        })
    }

    fn authenticate(&self) -> Result<Response, XError> {
        crate::requests::auth::Request::new(self).request()
    }
}

pub enum Authentication<'a> {
    GetBearerToken(&'a AppCredential),
}

impl<'a> Authentication<'a> {
    pub fn client(
        self,
        auth_url: AuthUrl,
        token_url: Option<TokenUrl>,
    ) -> Client<
        StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        BasicTokenType,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>,
    > {
        match self {
            Self::GetBearerToken(app_cred) => match app_cred {
                AppCredential::AppOnly {
                    client_id,
                    client_secret,
                } => BasicClient::new(
                    client_id.clone(),
                    Some(client_secret.clone()),
                    auth_url,
                    token_url,
                ),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
    fn create_basic_request_credential() {
        let authentication = AppCredential::AppOnly {
            client_id: oauth2::ClientId::new(String::from("7Vh2Xr8A35Vjn60eGfjaFqVeL")),
            client_secret: oauth2::ClientSecret::new(String::from(
                "zCEYKpHUJlBSzEET9uFrWoROl0rZGsCb1U2k4cCQY2clfb3WzI",
            )),
        };

        let request_credential = authentication.to_request_credential();

        assert!(request_credential.is_ok())
    }
}
