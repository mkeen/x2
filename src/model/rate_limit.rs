#[cfg(test)]
mod tests {
    use crate::{
        model::{
            auth::{AppCredential, RequestCredential},
            error::XError,
        },
        requests::Request,
    };

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

        if let Ok(auth) = authentication {
            println!(
                "{:?}",
                crate::requests::rate_limit::Request::new(auth).request()
            );
        }
    }
}
