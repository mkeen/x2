use crate::model::{auth::RequestCredential, error::XError};

#[derive(Clone)]
pub enum UsageScope {
    DailyClientAppUsage,
    DailyProjectUsage,
}

impl<'a> Into<&'a str> for UsageScope {
    fn into(self) -> &'a str {
        match self {
            Self::DailyProjectUsage => "daily_project_usage",
            Self::DailyClientAppUsage => "daily_client_app_usage",
        }
    }
}

pub struct Request<'a> {
    client: reqwest::blocking::Client,
    credential: RequestCredential<'a>,
    days: &'a str,
    // usage: UsageScope,
}

impl<'a> super::Request<'a> for Request<'a> {
    fn new(
        credential: RequestCredential,
        days: Option<&'a str>,
        //usage: Option<UsageScope>,
    ) -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .user_agent(super::APP_USER_AGENT)
                .referer(false)
                .https_only(true)
                .gzip(true)
                .build()
                .unwrap(),
            credential,
            days: days.unwrap_or("90"),
            // usage: usage.unwrap_or(UsageScope::DailyProjectUsage),
        }
    }

    #[allow(refining_impl_trait)]
    fn request(
        &self,
    ) -> Result<&crate::responses::usage_tweets::Response, crate::model::error::XError> {
        match self.credential {
            RequestCredential::Bearer(bearer) => {
                let params = &[("days", self.days)];

                let r = self
                    .client
                    .get(crate::config::Endpoint::UsageTweets.url())
                    .query(params)
                    .bearer_auth(bearer)
                    .send();

                crate::responses::usage_tweets::Response::try_from_bytes(
                    &r.map_err(|e| XError::Auth(e))?.bytes().unwrap(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            auth::{AppCredential, RequestCredential},
            error::XError,
        },
        requests::Request,
    };

    #[test]
    // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
    fn integration_usage_tweets_with_defaults() {
        let client_id = "gUJTmN2jcD7zOg2kFcbbS3fSp";
        let client_secret = "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon";

        let authentication: Result<RequestCredential, XError> = AppCredential::AppOnly {
            client_id,
            client_secret,
        }
        .try_into();

        if let Ok(auth) = authentication {
            let r = crate::requests::usage_tweets::Request::new(auth, None).request();
            //println!("{:?}", r);
            assert!(r.is_ok());
        } else {
            assert!(false);
        }
    }
}
