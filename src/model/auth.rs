use crate::requests::Request;

use super::error::XError;

#[derive(Debug)]
pub enum RequestCredential {
    Bearer(oauth2::AccessToken),
}

#[derive(Debug)]
pub enum AppCredential {
    AppOnly {
        client_id: oauth2::ClientId,
        client_secret: oauth2::ClientSecret,
    },
}

impl TryInto<RequestCredential> for AppCredential {
    type Error = XError;

    fn try_into(self) -> Result<RequestCredential, XError> {
        crate::requests::auth::Request::new(self).request()
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

        let request_credential: Result<RequestCredential, XError> = authentication.try_into();

        println!("{:?}", request_credential);

        assert!(request_credential.is_ok())
    }
}
