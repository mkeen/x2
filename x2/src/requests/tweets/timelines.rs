use chrono::{DateTime, Utc};
use model::EMPTY_STRING;

use super::prelude::*;

use crate::responses::tweets::lookup::Response;

#[derive(IntoStaticStr, Deserialize, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Exclude {
    Retweets,
    Replies,
}

#[derive(IntoStaticStr, Deserialize, EnumCount, Clone)]
pub enum Expansion {
    #[serde(rename = "attachments.poll_ids")]
    #[strum(serialize = "attachments.poll_ids")]
    AttachmentsPollIds,
    #[serde(rename = "attachments.media_keys")]
    #[strum(serialize = "attachments.media_keys")]
    AttachmentsMediaKeys,
    #[serde(rename = "author_id")]
    #[strum(serialize = "author_id")]
    AuthorId,
    #[serde(rename = "edit_history_tweet_ids")]
    #[strum(serialize = "edit_history_tweet_ids")]
    EditHistoryTweetIds,
    #[serde(rename = "entities.mentions.username")]
    #[strum(serialize = "entities.mentions.username")]
    EntitiesMentionUsername,
    #[serde(rename = "geo.place_id")]
    #[strum(serialize = "geo.place_id")]
    GeoPlaceId,
    #[serde(rename = "in_reply_to_user_id")]
    #[strum(serialize = "in_reply_to_user_id")]
    InReplyToUserId,
    #[serde(rename = "referenced_tweets.id")]
    #[strum(serialize = "referenced_tweets.id")]
    ReferencedTweetsId,
    #[serde(rename = "referenced_tweets.id.author_id")]
    #[strum(serialize = "referenced_tweets.id.author_id")]
    ReferencedTweetsIdAuthorId,
}

static DEFAULT_EXCLUDES: [Exclude; 0] = [];
static DEFAULT_FIELDS: [Field; 0] = [];
static DEFAULT_FIELDS_MEDIA: [MediaField; 0] = [];
static DEFAULT_FIELDS_PLACE: [PlaceField; 0] = [];
static DEFAULT_FIELDS_POLL: [PollField; 0] = [];
static DEFAULT_FIELDS_USER: [UserField; 0] = [];
static DEFAULT_EXPANSIONS: [Expansion; 0] = [];
const MAX_PARAM_MEMBERS: usize = 13;

pub struct Fields<'a> {
    tweets: &'a [Field],
    user: &'a [UserField],
    media: &'a [MediaField],
    place: &'a [PlaceField],
    poll: &'a [PollField],
}

impl<'a> Default for Fields<'a> {
    fn default() -> Self {
        Self {
            tweets: &DEFAULT_FIELDS,
            user: &DEFAULT_FIELDS_USER,
            media: &DEFAULT_FIELDS_MEDIA,
            place: &DEFAULT_FIELDS_PLACE,
            poll: &DEFAULT_FIELDS_POLL,
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
        exclude: Option<&[Exclude]>,
        max_results: Option<usize>,
        since_id: Option<&str>,
        until_id: Option<&str>,
        start_time: Option<DateTime<Utc>>,
        pagination_token: Option<&str>,
    ) -> Self {
        let fields = fields.unwrap_or_default();

        let expansions = expansions.unwrap_or(&DEFAULT_EXPANSIONS);

        let max_results = format!("{}", max_results.unwrap_or(10));
        let pagination_token = pagination_token.unwrap_or(&EMPTY_STRING);
        let exclude = exclude.unwrap_or(&DEFAULT_EXCLUDES);
        let since_id = since_id.unwrap_or(&EMPTY_STRING);
        let until_id = until_id.unwrap_or(&EMPTY_STRING);
        let start_time = start_time
            .map(|d| d.to_rfc3339())
            .unwrap_or(EMPTY_STRING.clone());

        let fixed_query: [(String, String); MAX_PARAM_MEMBERS] = [
            ("id".into(), id.into()),
            ("expansions".into(), csv(expansions)),
            ("exclude".into(), csv(exclude)),
            ("pagination_token".into(), pagination_token.into()),
            ("since_id".into(), since_id.into()),
            ("until_id".into(), until_id.into()),
            ("start_time".into(), start_time),
            ("max_results".into(), max_results),
            ("tweet.fields".into(), csv(fields.tweets)),
            ("user.fields".into(), csv(fields.user)),
            ("media.fields".into(), csv(fields.media)),
            ("place.fields".into(), csv(fields.place)),
            ("poll.fields".into(), csv(fields.poll)),
        ];

        Self {
            builder: Some(RequestBuilder::Oauth1(
                Self::authorize_oauth1(auth)
                    .get(super::Endpoint::Lookup.url(None))
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
    use model::auth::RequestCredential;

    use crate::test_util::oauth1_credentials;

    use super::*;

    #[test]
    fn get_tweet_timelines() {
        let context = oauth1_credentials();

        let mut response = Request::new(
            &context, &"---", None, None, None, None, None, None, None, None,
        );

        // request has been built and is able to be sent
        match response.builder.take() {
            Some(builder) => match builder {
                requests::ClientAgnosticBuilder::Native(_) => assert!(false),
                requests::ClientAgnosticBuilder::Oauth1(builder) => {
                    assert!(true)
                }
            },

            None => {
                assert!(false)
            }
        };
    }
}
