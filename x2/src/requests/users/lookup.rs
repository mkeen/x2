use bytes::Bytes;

use super::prelude::*;
use crate::{model::spaces::Field, responses::users::Response};

#[derive(IntoStaticStr, Deserialize, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(serialize = "pinned_tweet_id")]
    PinnedTweetId,
}

static DEFAULT_FIELDS_USER: [Field; 0] = [];
static DEFAULT_FIELDS_TWEETS: [TweetField; 0] = [];

pub struct Fields<'a> {
    user: &'a [Field],
    tweets: &'a [TweetField],
}

impl<'a> Default for Fields<'a> {
    fn default() -> Self {
        Self {
            user: &DEFAULT_FIELDS_USER,
            tweets: &DEFAULT_FIELDS_TWEETS,
        }
    }
}

pub struct Request {
    builder: Option<RequestBuilder>,
}

impl Authorized<Response> for Request {}

impl<'a> Request {
    pub fn new(
        auth: &'a Context,
        usernames: &[&str],
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
    ) -> Self {
        let fields = fields.unwrap_or_default();

        Self {
            builder: Self::builder_with_auth(
                auth,
                client()
                    .get(crate::config::Endpoint::UserLookup.url(None))
                    .query(&[
                        ("usernames", usernames.join(",")),
                        ("expansions", csv(expansions.unwrap_or(&[]))),
                        ("user.fields", csv(fields.user)),
                        ("tweet.fields", csv(fields.tweets)),
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
        requests::{users::lookup::*, Request as RequestTrait},
    };

    #[test]
    fn integration_users_lookup_with_defaults() {
        let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
        let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

        let context = auth::Context::Caller(auth::Method::AppOnly { id, secret });

        // not testing authentication here, so will just unwrap and assume all is well
        let authorization = context.authorize().unwrap();

        let response =
            Request::new(&authorization, &["divxspan", "wamalone"], None, None).request();

        assert!(response.is_ok());

        let response = response.unwrap();

        assert!(!response.data.is_empty())
    }
}
