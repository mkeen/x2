use super::prelude::*;

use responses::users::muting::lookup::Response;

#[derive(IntoStaticStr, Deserialize, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(serialize = "pinned_tweet_id")]
    PinnedTweetId,
}

static DEFAULT_FIELDS: [Field; 0] = [];
static DEFAULT_FIELDS_TWEETS: [TweetField; 0] = [];
static DEFAULT_EXPANSIONS: [Expansion; 0] = [];
const MAX_PARAM_MEMBERS: usize = 5;

pub struct Fields<'a> {
    user: &'a [Field],
    tweets: &'a [TweetField],
}

impl<'a> Default for Fields<'a> {
    fn default() -> Self {
        Self {
            user: &DEFAULT_FIELDS,
            tweets: &DEFAULT_FIELDS_TWEETS,
        }
    }
}

#[derive(Debug, Built, Authorized)]
pub struct Request<'a> {
    builder: Option<RequestBuilder<'a>>,
}

impl<'a> Request<'a> {
    pub fn new(
        auth: &'a Context,
        id: &str,
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
        max_results: Option<u16>,
        pagination_token: Option<&str>,
    ) -> Self {
        let fields = fields.unwrap_or_default();
        let pagination_token = pagination_token.unwrap_or("");

        let expansions = csv(expansions.unwrap_or(&DEFAULT_EXPANSIONS));
        let max_results = format!("{}", max_results.unwrap_or(DEFAULT_RESULT_LIMIT));

        let fixed_query: [(String, String); MAX_PARAM_MEMBERS] = [
            ("expansions".into(), expansions),
            ("max_results".into(), max_results),
            ("user.fields".into(), csv(fields.user)),
            ("tweet.fields".into(), csv(fields.tweets)),
            ("pagination_token".into(), pagination_token.into()),
        ];

        Self {
            builder: Some(RequestBuilder::Oauth1(
                Self::authorize_oauth1(auth)
                    .get(super::Endpoint::Lookup.url(Some(&[id])))
                    .query(
                        &fixed_query
                            .iter()
                            .filter(|(_, param_entry)| !param_entry.is_empty())
                            .collect::<Vec<&(String, String)>>(),
                    ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::auth::{Method, RequestCredential},
        test_util::oauth1_credentials,
    };

    use super::*;

    #[test]
    fn muting_lookup() {
        let context = oauth1_credentials();

        let response =
            Request::new(&context, "1444148135954108418", None, None, None, None).request();

        assert!(response.is_ok());

        let response = response.unwrap();
        assert!(!response.data.is_empty());
    }
}
