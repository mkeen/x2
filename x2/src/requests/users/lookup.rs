use super::prelude::*;
use crate::{model::spaces::Field, responses::users::lookup::Response};

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

#[derive(Debug, Built, Authorized)]
pub struct Request {
    builder: Option<RequestBuilder>,
}

impl Request {
    pub fn new(
        auth: &Context,
        usernames: &[&str],
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
    ) -> Self {
        let fields = fields.unwrap_or_default();

        Self {
            builder: Self::authorize_simple(
                auth,
                client().get(super::Endpoint::Lookup.url(None)).query(&[
                    ("usernames", usernames.join(",")),
                    ("expansions", csv(expansions.unwrap_or(&[]))),
                    ("user.fields", csv(fields.user)),
                    ("tweet.fields", csv(fields.tweets)),
                ]),
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::model::auth::Method;

    use super::*;

    #[test]
    fn integration_users_lookup_with_defaults() {
        let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
        let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

        let context = Context::Caller(Method::AppOnly { id, secret });

        let authorization: Context = context.authenticate().unwrap();

        let response =
            Request::new(&authorization, &["divxspan", "wamalone"], None, None).request();

        assert!(response.is_ok());

        println!("{:?}", response.unwrap().data);
    }
}
