use model::EMPTY_STRING;

use super::prelude::*;

use crate::responses::tweets::bookmarks::lookup::Response;

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
const MAX_PARAM_MEMBERS: usize = 8;

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
pub struct Request {
    builder: Option<RequestBuilder>,
}

impl Request {
    pub fn new(
        auth: &Context,
        user_id: &str,
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
        max_results: Option<usize>,
        pagination_token: Option<&str>,
    ) -> Self {
        let fields = fields.unwrap_or_default();

        let expansions = expansions.unwrap_or(&DEFAULT_EXPANSIONS);

        let max_results = format!("{}", max_results.unwrap_or(10));
        let pagination_token = pagination_token.unwrap_or(&EMPTY_STRING);

        let fixed_query: [(String, String); MAX_PARAM_MEMBERS] = [
            ("expansions".into(), csv(expansions)),
            ("pagination_token".into(), pagination_token.into()),
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
                    .get(super::Endpoint::Lookup.url(Some(&[user_id])))
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
    use crate::test_util::oauth1_credentials;

    use super::*;

    fn get_bookmarks_lookup() {
        let context = oauth1_credentials();

        let mut response = Request::new(&context, &"---", None, None, None, None);

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
