use super::prelude::*;

use crate::responses::users::Response;

#[derive(IntoStaticStr, Deserialize, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(serialize = "pinned_tweet_id")]
    PinnedTweetId,
}

static DEFAULT_FIELDS: [Field; 0] = [];
static DEFAULT_FIELDS_TWEETS: [TweetField; 0] = [];
static DEFAULT_EXPANSIONS: [Expansion; 0] = [];

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
pub struct Request {
    builder: Option<RequestBuilder>,
}

impl<'a> Request {
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

        Self {
            builder: Self::authorize(
                auth,
                client()
                    .get(super::Endpoint::Blocking.url(Some(&[id])))
                    .query(&[
                        ("expansions", expansions.as_str()),
                        ("max_results", max_results.as_str()),
                        ("user.fields", csv(fields.user).as_str()),
                        ("tweet.fields", csv(fields.tweets).as_str()),
                        ("pagination_token", pagination_token),
                    ]),
            ),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::model::auth::Method;

//     use super::*;

//     #[test]
//     fn integration_users_blocking_lookup_with_defaults() {
//         let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
//         let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

//         let context = Context::Caller(Method::AppOnly { id, secret });
//         let response = Request::new(&context, "123", None, None, None, None).request();

//         assert!(response.is_ok());
//         assert!(response.unwrap().is_bearer());

//         println!("{:?}", response);

//         assert!(response.is_ok());

//         let response = response.unwrap();

//         assert!(!response.data.is_empty())
//     }
// }
