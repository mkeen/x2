use serde::Deserialize;
use strum::AsRefStr;

use crate::{
    model::{auth::*, error::XError, tweets::Field as TweetField, users::Field as UserField},
    requests::{fields_as_csv, strings_as_csv},
    responses::*,
};

#[derive(AsRefStr, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(serialize = "pinned_tweet_id")]
    PinnedTweetId,
}

pub struct Fields<'a> {
    user: Vec<&'a UserField>,
    tweets: Vec<&'a TweetField>,
}

pub struct Params<'a> {
    usernames: Vec<&'a str>,
}

impl<'a> Default for Params<'a> {
    fn default() -> Self {
        Self { usernames: vec![] }
    }
}

pub struct Options<'a> {
    params: &'a Params<'a>,
    expansions: Option<Vec<&'a Expansion>>,
    fields: Option<&'a Fields<'a>>,
}

static DEFAULT_OPTION_PARAMS: Params = Params { usernames: vec![] };

impl<'a> Options<'a> {
    fn default() -> Self {
        Self {
            params: &DEFAULT_OPTION_PARAMS,
            expansions: None,
            fields: None,
        }
    }
}

pub struct Request<'a> {
    client: &'a reqwest::blocking::Client,
    credential: &'a RequestCredential,
    params: Vec<(&'a str, &'a str)>,
}

impl<'a> Request<'a> {
    pub fn new(credential: &Credential, options: Option<Options>) -> Self {
        let options = options.unwrap_or(Options::default());
        let fields = options.fields.unwrap(); // this unwrap should be safe -- todo -- take a look

        let mut params: Vec<(&str, &str)> = Vec::with_capacity(4);

        params.push((
            "usernames",
            strings_as_csv(&options.params.usernames).as_str(),
        ));

        push_to_params(
            &mut params,
            fields_as_csv(&options.expansions.unwrap_or_default()).as_str(),
            "expansions",
        );
        push_to_params(
            &mut params,
            fields_as_csv(&fields.tweets).as_str(),
            "tweet.fields",
        );
        push_to_params(
            &mut params,
            fields_as_csv(&fields.user).as_str(),
            "user.fields",
        );

        params.shrink_to_fit();

        Self {
            client: super::super::client(),
            credential: credential
                .try_into()
                .expect("could not authorize credentials"),
            params,
        }
    }
}

impl<'a> super::Request<'a> for Request<'a> {
    type Response = crate::responses::users::lookup::Response;

    fn request(&self) -> Result<Self::Response, XError> {
        match &self.credential {
            RequestCredential::Bearer(bearer) => {
                let result = self
                    .client
                    .get(crate::config::Endpoint::UserLookup.url())
                    .query(&self.params)
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
            client_id: "gUJTmN2jcD7zOg2kFcbbS3fSp".into(),
            client_secret: "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon".into(),
        });

        let result = crate::requests::users::lookup::Request::new(
            &credential,
            Some(Options {
                params: &crate::requests::users::lookup::Params {
                    usernames: vec!["divxspan"],
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
