use super::prelude::*;

use crate::responses::tweets::lookup::Response;

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

static DEFAULT_FIELDS: [Field; 0] = [];
static DEFAULT_FIELDS_MEDIA: [MediaField; 0] = [];
static DEFAULT_FIELDS_PLACE: [PlaceField; 0] = [];
static DEFAULT_FIELDS_POLL: [PollField; 0] = [];
static DEFAULT_FIELDS_USER: [UserField; 0] = [];
static DEFAULT_EXPANSIONS: [Expansion; 0] = [];
const MAX_PARAM_MEMBERS: usize = 7;

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
        tweet_ids: &[&str],
        expansions: Option<&[Expansion]>,
        fields: Option<Fields>,
    ) -> Self {
        let fields = fields.unwrap_or_default();

        let expansions = csv(expansions.unwrap_or(&DEFAULT_EXPANSIONS));

        let fixed_query: [(String, String); MAX_PARAM_MEMBERS] = [
            ("ids".into(), tweet_ids.join(",").to_string()),
            ("expansions".into(), expansions),
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
    use crate::{
        model::auth::{Method, RequestCredential},
        requests,
    };

    use requests::users::lookup::Fields as UserFields;

    use super::*;

    #[test]
    fn lookup_tweets<'a>() {
        let consumer_id = "c2HAMlWTX2m3cVgNgA0oqLRqH".to_string();
        let consumer_secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2".to_string();

        let oauth2_client_id = "TV9xZXRVVVN0STIwSkcwck9WS2w6MTpjaQ".to_string();
        let oauth2_client_secret = "gZHqK9YQZyrH7x7P9Yg5kxdE3j8_yDQopjBxXIptw-4b2TIM4_".to_string();

        let user_id = "1444148135954108418-TSUe6cI1lpIddYScxSKIlmbfq71kyL".to_string();
        let user_secret = "vupepUIBVJl08dhMdlHuNTyRWaWUVPenrPpSl1E4EqWb6".to_string();

        let a = consumer_id.clone();
        let b = consumer_secret.clone();

        // let user_context = Context::Caller(Method::AppOnly { id: &a, secret: &b });

        // let authorized_user_context = user_context.authenticate().unwrap();

        let context = Context::Request(RequestCredential::OAuth10AConsumer {
            consumer_id,
            consumer_secret,
            user_id,
            user_secret,
        });

        // let mut user_f = UserFields::default();
        // user_f.user = &[UserField::Id];

        // let p = requests::users::lookup::Request::new(
        //     &authorized_user_context,
        //     &["divxspan"],
        //     Some(&[requests::users::lookup::Expansion::PinnedTweetId]),
        //     Some(user_f),
        // )
        // .request()
        // .unwrap();

        // print!("pppppppppp {:?} ppppppp", p.includes);

        // let user_r = p.data;

        // let pinned_tweet_id = user_r.first().unwrap().pinned_tweet_id().unwrap();
        // println!("wefwefwefwef!!! {:?}", user_r);

        let response = Request::new(
            &context,
            &["1819793989618315627", "1819801049940816075"],
            Some(&[Expansion::ReferencedTweetsId]),
            None,
        )
        .request();
        // todo this is a bad id here

        println!("ewwfwe {:?}", response);

        assert!(response.is_ok());
    }
}
