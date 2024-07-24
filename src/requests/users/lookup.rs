use serde::Deserialize;
use strum::{Display, EnumString};

use crate::{
    model::{auth::*, error::XError, tweets::Field as TweetField, users::Field as UserField},
    requests::{fields_as_csv, push_to_params},
    responses::*,
};

#[derive(EnumString, Display, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(to_string = "pinned_tweet_id")]
    PinnedTweetId,
}

#[derive(Default)]
pub struct Fields {
    user: Vec<UserField>,
    tweets: Vec<TweetField>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    usernames: Vec<String>,
}

pub struct Options {
    params: Params,
    expansions: Option<Vec<Expansion>>,
    fields: Option<Fields>,
}

impl<'a> Default for Options {
    fn default() -> Self {
        Self {
            params: Params { usernames: vec![] },
            expansions: None,
            fields: None,
        }
    }
}

pub struct Request<'a> {
    client: &'a reqwest::blocking::Client,
    credential: RequestCredential,
    usernames: String,
    expansions: String,
    tweet_fields: String,
    user_fields: String,
}

impl<'a> Request<'a> {
    pub fn new(credential: &Credential, options: Option<Options>) -> Self {
        let options = options.unwrap_or_default();

        let fields = &options.fields.unwrap_or_default();

        Self {
            client: super::super::client(),
            credential: credential
                .try_into()
                .expect("could not authorize credentials"),
            usernames: options.params.usernames.join(","),
            expansions: fields_as_csv(&options.expansions.unwrap_or_default()),
            tweet_fields: fields_as_csv(&fields.tweets),
            user_fields: fields_as_csv(&fields.user),
        }
    }
}

impl<'a> super::Request<'a> for Request<'a> {
    type Response = crate::responses::users::lookup::Response;

    fn request(&self) -> Result<Self::Response, XError> {
        match &self.credential {
            RequestCredential::Bearer(bearer) => {
                let mut params: Vec<(String, String)> = Vec::with_capacity(4);
                params.push(("usernames".into(), self.usernames.clone()));

                push_to_params(&mut params, &self.expansions, "expansions");
                push_to_params(&mut params, &self.tweet_fields, "tweet.fields");
                push_to_params(&mut params, &self.user_fields, "user.fields");

                params.shrink_to_fit();

                let params = params;

                let result = self
                    .client
                    .get(crate::config::Endpoint::UserLookup.url())
                    .query(&params)
                    .bearer_auth(bearer)
                    .send()
                    .map_err(|e| XError::Generic(e.status().unwrap_or_default(), e.to_string()))?;

                match result.status().is_success() {
                    true => Self::Response::try_into_from_bytes(
                        &result.bytes().map_err(|e| XError::Reqwest(e))?,
                    ),
                    false => Err(XError::Generic(
                        result.status(),
                        result.text().unwrap_or("Unknown".into()),
                    )),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::auth::AppCredential,
        requests::{users::lookup::Options, Request},
    };

    use super::Credential;

    #[test]
    fn integration_users_lookup_with_defaults() {
        let credential = Credential::Unauthorized(AppCredential::AppOnly {
            client_id: "gUJTmN2jcD7zOg2kFcbbS3fSp",
            client_secret: "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon",
        });

        let result = crate::requests::users::lookup::Request::new(
            &credential,
            Some(Options {
                params: crate::requests::users::lookup::Params {
                    usernames: vec![String::from("divxspan")],
                },
                fields: None,
                expansions: None,
            }),
        )
        .request();

        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
