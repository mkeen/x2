use super::prelude::*;

use crate::responses::users::lookup::Response as UserLookupResponse;

#[derive(AsRefStr, Deserialize, EnumCount)]
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
    builder: RequestBuilder,
}

impl<'a> Request {
    pub fn new(
        auth: &'a Context,
        usernames: &[&str],
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
    ) -> Self {
        let fields = fields.unwrap_or_default();
        let expansions = collect_csv::<Expansion, { Expansion::COUNT }>(expansions.unwrap_or(&[]));
        let user_fields = collect_csv::<Field, { Field::COUNT }>(fields.user);
        let tweet_fields = collect_csv::<TweetField, { TweetField::COUNT }>(fields.tweets);

        Self {
            builder: Self::builder_with_auth(
                auth,
                client()
                    .get(crate::config::Endpoint::UserLookup.url())
                    .query(&[
                        ("usernames", usernames.join(",")),
                        ("expansions", expansions),
                        ("user.fields", user_fields),
                        ("tweet.fields", tweet_fields),
                    ]),
            ),
        }
    }
}

impl Authorized<UserLookupResponse> for Request {}

impl<'a> super::Request<UserLookupResponse> for Request {
    fn request(self) -> Result<UserLookupResponse, XError> {
        self.builder
            .send()
            .map_err(|e| XError::Socket(e.to_string()))
            .map(|response| match response.status().is_success() {
                true => UserLookupResponse::try_into_from_bytes(
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
        requests::{users::lookup::*, Request as RequestTrait},
    };

    #[test]
    fn integration_users_lookup_with_defaults() {
        let id = "GJe6IFjFNwveQzBhJmaMIZzW5";
        let secret = "f9kmkg3eQkxNB7thibc5lhhgavCq4eQmMrTdeO9aw4rIz4Hofb";

        let context = auth::Context::Caller(auth::Method::AppOnly { id, secret });

        // not testing authentication here, so will just unwrap and assume all is well
        let authorization = context.authorize().unwrap();

        let response =
            Request::new(&authorization, &["divxspan", "wamalone"], None, None).request();

        assert!(response.is_ok());

        let response = response.unwrap();

        assert!(!response.users().is_empty())
    }
}
