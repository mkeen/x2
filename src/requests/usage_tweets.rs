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
    credential: RequestCredential,
    days: &'a str,
    // usage: UsageScope,
}

impl<'a> Request<'a> {
    pub fn new(
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
}

impl<'a> super::Request<'a> for Request<'a> {
    #[allow(refining_impl_trait)]
    fn request(
        &self,
    ) -> Result<crate::responses::usage_tweets::Response, crate::model::error::XError> {
        match &(self.credential) {
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
