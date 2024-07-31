use super::prelude::*;

use crate::{
    model::{topics::Field as TopicField, users::Field as UserField},
    responses::spaces::search::Response as SearchResponse,
};

use rayon::prelude;

use super::Authorized;

#[derive(IntoStaticStr, Deserialize, EnumCount, Clone)]
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

static DEFAULT_FIELDS_SPACE: [Field; 0] = [];
static DEFAULT_FIELDS_TOPIC: [TopicField; 0] = [];
static DEFAULT_FIELDS_USER: [UserField; 0] = [];

pub struct Fields<'a> {
    pub space: &'a [Field],
    pub topic: &'a [TopicField],
    pub user: &'a [UserField],
}

impl<'a> Default for Fields<'a> {
    fn default() -> Self {
        Self {
            space: &DEFAULT_FIELDS_SPACE,
            topic: &DEFAULT_FIELDS_TOPIC,
            user: &DEFAULT_FIELDS_USER,
        }
    }
}

pub struct Request {
    builder: RequestBuilder,
}

impl<'a> Request {
    pub fn new(
        auth: &'a Context,
        query: &'a str,
        state: State,
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
    ) -> Self {
        let fields = fields.unwrap_or_default();
        let expansions = expansions.unwrap_or_default();

        let expansions = csv::<{ Expansion::COUNT }, Expansion>(expansions);
        let fields_space = csv::<{ Field::COUNT }, Field>(fields.space);
        let fields_user = csv::<{ UserField::COUNT }, UserField>(fields.user);
        let fields_topic = csv::<{ TopicField::COUNT }, TopicField>(fields.topic);

        Self {
            builder: Self::builder_with_auth(
                auth,
                super::super::client()
                    .get(crate::config::Endpoint::SpacesSearch.url())
                    .query(&[
                        ("query", query),
                        ("state", state.into()),
                        ("expansions", &expansions),
                        ("space.fields", &fields_space),
                        ("user.fields", &fields_user),
                        ("topic.fields", &fields_topic),
                    ]),
            ),
        }
    }
}

impl Authorized<SearchResponse> for Request {}

impl<'a> super::Request<SearchResponse> for Request {
    fn request(self) -> Result<SearchResponse, XError> {
        self.builder
            .send()
            .map_err(|e| XError::Socket(e.to_string()))
            .map(|response| match response.status().is_success() {
                true => SearchResponse::try_into_from_bytes(
                    &response.bytes().map_err(|e| XError::Reqwest(e))?,
                ),

                false => Err(XError::HttpGeneric(
                    response.status(),
                    response.text().unwrap_or("Unknown".into()),
                )),
            })?
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::auth,
        requests::{spaces::search::*, Request as RequestTrait},
    };

    #[test]
    fn integration_spaces_search_with_defaults() {
        let id = "gUJTmN2jcD7zOg2kFcbbS3fSp";
        let secret = "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon";

        let context = auth::Context::Caller(auth::Method::AppOnly { id, secret });

        // not testing authentication here, so will just unwrap and assume all is well
        let authorization = context.authorize().unwrap();

        let response = Request::new(&authorization, "crypto", State::All, None, None).request();

        assert!(response.is_ok());

        let response = response.unwrap();

        assert!(!response.spaces().is_empty());
    }
}
