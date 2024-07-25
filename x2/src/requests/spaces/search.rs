use arrayvec::ArrayVec;
use serde::Deserialize;
use strum::{AsRefStr, EnumCount};
use x2_derive::request;

use crate::{
    model::{
        auth::*, error::XError, spaces::*, topics::Field as TopicField, users::Field as UserField,
    },
    responses::spaces::search::*,
};

#[derive(AsRefStr, Deserialize, EnumCount)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(serialize = "invited_user_ids")]
    InvitedUserIds,
    #[strum(serialize = "speaker_ids")]
    SpeakerIds,
    #[strum(serialize = "creator_ids")]
    CreatorIds,
    #[strum(serialize = "host_ids")]
    HostIds,
    #[strum(serialize = "topics_ids")]
    TopicsIds,
}

pub struct Fields<'a> {
    space: &'a [Field],
    topic: &'a [TopicField],
    user: &'a [UserField],
}

pub struct Request<'a> {
    client: &'a reqwest::blocking::Client,
    credential: &'a Credential,
    authorization: Option<&'a RequestCredential>,
    params: [(&'a str, &'a str); 6],
}

impl<'a> Request<'a> {
    fn new(
        credential: &'a Credential,
        query: &str,
        state: Option<State>,
        expansions: Option<&[Expansion]>,
        fields: Option<&Fields>,
    ) -> Self {
        let params: [(&str, &str); 6] = [
            // todo dont send all args if some are empty
            ("query", query),
            ("state", state.unwrap_or(State::All).as_ref()),
            (
                "expansions",
                match expansions {
                    Some(expansions) => &expansions
                        .iter()
                        .map(|e| e.as_ref())
                        .collect::<ArrayVec<&str, 5>>() // arg, can't use Expansion::COUNT -- todo! // and i have to use a ArrayVec since there's no [] iter collect support :(
                        .join(","),
                    None => "",
                },
            ),
            (
                "space.fields",
                match fields {
                    Some(fields) => &fields
                        .space
                        .iter()
                        .map(|e| e.as_ref())
                        .collect::<ArrayVec<&str, 17>>() // arg, can't use Field::COUNT -- todo!
                        .join(","),
                    None => "",
                },
            ),
            (
                "user.fields",
                match fields {
                    Some(fields) => &fields
                        .user
                        .iter()
                        .map(|e| e.as_ref())
                        .collect::<ArrayVec<&str, 16>>() // arg, can't use Field::COUNT -- todo!
                        .join(","),
                    None => "",
                },
            ),
            (
                "topic.fields",
                match fields {
                    Some(fields) => &fields
                        .topic
                        .iter()
                        .map(|e| e.as_ref())
                        .collect::<ArrayVec<&str, 3>>() // arg, can't use Field::COUNT -- todo!
                        .join(","),
                    None => "",
                },
            ),
        ];

        Self {
            client: super::super::client(),
            credential,
            authorization: None,
            params,
        }
    }
}

#[request]
impl<'a> super::Request<'a> for Request<'a> {
    fn authorize(&mut self) -> Result<(), XError> {
        match self.credential.try_into() {
            Ok(request_credential) => {
                self.authorization = request_credential;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn request(&self) -> Result<Self::Response, XError> {
        match &self.credential {
            RequestCredential::Bearer(bearer) => {
                let result = self
                    .client
                    .get(crate::config::Endpoint::SpacesSearch.url())
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
    use crate::requests::{spaces::search::*, Request as RequestTrait};

    #[test]
    fn integration_spaces_search_with_defaults() {
        let credential = Credential::Unauthorized(AppCredential::AppOnly {
            client_id: String::from("gUJTmN2jcD7zOg2kFcbbS3fSp"),
            client_secret: "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon".into(),
        });

        let result = Request::new(
            &credential,
            Some(Options {
                params: &Params {
                    query: "sports",
                    state: &State::All,
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
