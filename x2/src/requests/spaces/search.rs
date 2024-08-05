use super::prelude::*;

use crate::{
    model::{topics::Field as TopicField, users::Field as UserField},
    responses::spaces::search::Response,
};

const MAX_PARAM_MEMBERS: usize = 6;

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

#[derive(Debug, Built, Authorized)]
pub struct Request {
    builder: Option<super::RequestBuilder>,
}

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

        let state: &str = state.into();

        let fixed_query: [(String, String); MAX_PARAM_MEMBERS] = [
            ("query".into(), query.into()),
            ("state".into(), state.into()),
            ("expansions".into(), csv(expansions)),
            ("space.fields".into(), csv(fields.space)),
            ("user.fields".into(), csv(fields.user)),
            ("topic.fields".into(), csv(fields.topic)),
        ];

        Self {
            builder: Self::authorize_simple(
                auth,
                super::super::client()
                    .get(super::Endpoint::Search.url(None))
                    .query(
                        &fixed_query
                            .iter()
                            .filter(|(_, param_entry)| !param_entry.is_empty())
                            .collect::<Vec<&(String, String)>>(),
                    ),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::auth::Method;

    #[test]
    fn integration_spaces_search_with_defaults() {
        let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
        let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

        let context = Context::Caller(Method::AppOnly {
            id: id.into(),
            secret: secret.into(),
        });

        // not testing authentication here, so will just unwrap and assume all is well
        let context = context.authenticate().unwrap();

        let response = Request::new(&context, "crypto", State::All, None, None).request();

        assert!(response.is_ok());

        let response = response.unwrap();

        assert!(!response.data.is_empty());
    }
}
