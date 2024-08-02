use super::prelude::*;

use crate::{
    model::{topics::Field as TopicField, users::Field as UserField},
    responses::spaces::search::Response,
};

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
    builder: Option<RequestBuilder>,
}

impl Authorized<Response> for Request {}

impl Request {
    pub fn new(
        auth: &Context,
        query: &str,
        state: State,
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
    ) -> Self {
        let fields = fields.unwrap_or_default();
        let expansions = expansions.unwrap_or_default();

        let expansions = csv(expansions);
        let fields_space = csv(fields.space);
        let fields_user = csv(fields.user);
        let fields_topic = csv(fields.topic);

        Self {
            builder: Self::builder_with_auth(
                auth,
                super::super::client()
                    .get(Endpoint::SpacesSearch.url(None))
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

impl super::Request<Response> for Request {
    fn builder(&mut self) -> Option<RequestBuilder> {
        self.builder.take()
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
        let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
        let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

        let context = auth::Context::Caller(auth::Method::AppOnly { id, secret });

        // not testing authentication here, so will just unwrap and assume all is well
        let authorization = context.authorize().unwrap();

        let response = Request::new(&authorization, "crypto", State::All, None, None).request();

        assert!(response.is_ok());

        let response = response.unwrap();

        assert!(!response.data.is_empty());
    }
}
