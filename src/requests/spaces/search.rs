use serde::Deserialize;
use strum::{Display, EnumString};

use crate::{
    model::{
        auth::*, error::XError, spaces::*, topics::Field as TopicField, users::Field as UserField,
    },
    requests::fields_as_csv,
    responses::spaces::search::*,
};

use super::push_to_params;

#[derive(EnumString, Display, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(to_string = "invited_user_ids")]
    InvitedUserIds,
    #[strum(to_string = "speaker_ids")]
    SpeakerIds,
    #[strum(to_string = "creator_ids")]
    CreatorIds,
    #[strum(to_string = "host_ids")]
    HostIds,
    #[strum(to_string = "topics_ids")]
    TopicsIds,
}

#[derive(Default)]
pub struct Fields {
    space: Vec<UserField>,
    topic: Vec<TopicField>,
    user: Vec<UserField>,
}

pub struct Params {
    query: String,
    state: State,
}

pub struct Options {
    params: Params,
    expansions: Option<Vec<Expansion>>,
    fields: Option<Fields>,
}

impl<'a> Default for Options {
    fn default() -> Self {
        Self {
            params: Params {
                query: String::from(""),
                state: State::All,
            },
            expansions: None,
            fields: None,
        }
    }
}

pub struct Request<'a> {
    client: &'a reqwest::blocking::Client,
    credential: RequestCredential,
    query: String,
    expansions: String,
    space_fields: String,
    user_fields: String,
    topic_fields: String,
    state: String,
}

impl<'a> Request<'a> {
    fn new(credential: &'a Credential, options: Option<Options>) -> Self {
        let options = options.unwrap_or_default();
        let fields = &options.fields.unwrap_or_default();

        Self {
            client: super::super::client(),
            credential: credential
                .try_into()
                .expect("could not authorize credentials"),
            query: options.params.query,
            expansions: fields_as_csv(&options.expansions.unwrap_or_default()),
            space_fields: fields_as_csv(&fields.space),
            user_fields: fields_as_csv(&fields.user),
            topic_fields: fields_as_csv(&fields.topic),
            state: options.params.state.to_string(),
        }
    }
}

impl<'a> super::Request<'a> for Request<'a> {
    type Response = Response;

    fn request(&self) -> Result<Self::Response, XError> {
        match &self.credential {
            RequestCredential::Bearer(bearer) => {
                let mut params: Vec<(String, String)> = Vec::with_capacity(4);
                params.push(("query".into(), self.query.clone()));

                push_to_params(&mut params, &self.expansions, "expansions");
                push_to_params(&mut params, &self.space_fields, "space.fields");
                push_to_params(&mut params, &self.user_fields, "user.fields");
                push_to_params(&mut params, &self.topic_fields, "topic.fields");
                push_to_params(&mut params, &self.state, "state");

                params.shrink_to_fit();

                let result = self
                    .client
                    .get(crate::config::Endpoint::SpacesSearch.url())
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
        model::spaces::*,
        requests::{spaces::search::*, Request as RequestTrait},
    };

    #[test]
    fn integration_spaces_search_with_defaults() {
        let credential = Credential::Unauthorized(AppCredential::AppOnly {
            client_id: "gUJTmN2jcD7zOg2kFcbbS3fSp",
            client_secret: "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon",
        });

        let result = Request::new(
            &credential,
            Some(Options {
                params: Params {
                    query: "sports".into(),
                    state: State::All,
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
