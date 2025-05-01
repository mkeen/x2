use super::prelude::{Deserialize, EnumCount, IntoStaticStr, XData};

use chrono::{DateTime, Utc};

use super::{
    entities::{DomainEntityInfo, Entities, Entity},
    media::Media,
    places::Place,
    polls::Poll,
    users::User,
    withheld::Withheld,
};

#[derive(Debug, Deserialize, XData)]
pub struct EditControls {
    is_edit_eligible: Option<bool>,
    editable_until: Option<DateTime<Utc>>,
    edits_remaining: Option<u8>,
}

#[derive(Debug, Deserialize, XData)]
pub struct NoteTweet {
    text: Option<String>,
    entities: Option<Vec<Entities>>,
}

#[derive(Debug, Deserialize, XData)]
pub struct Attachment {
    media_keys: Option<Vec<String>>,
    poll_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, XData)]
pub struct Coordinates {
    #[serde(rename = "type")]
    _type: Option<String>,
    coordinates: Option<(f32, f32)>,
    place_id: Option<String>,
}

#[derive(Debug, Deserialize, XData)]
pub struct Geo {
    coordinates: Option<Coordinates>,
}

#[derive(Debug, Deserialize, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
pub enum ContextAnnotations {
    #[strum(serialize = "domain")]
    Domain(Option<Vec<DomainEntityInfo>>),
    #[strum(serialize = "entity")]
    Entity(Option<Vec<Entity>>),
}

#[derive(Debug, Deserialize, XData)]
pub struct PublicMetrics {
    retweet_count: Option<u64>,
    reply_count: Option<u64>,
    like_count: Option<u64>,
    quote_count: Option<u64>,
    impression_count: Option<u64>,
    bookmark_count: Option<u64>,
}

#[derive(Debug, Deserialize, XData)]
pub struct NonpublicMetrics {
    impression_count: Option<u64>,
    url_link_clicks: Option<u64>,
    user_profile_clicks: Option<u64>,
}

#[derive(Debug, Deserialize, XData)]
pub struct OrganicMetrics {
    impression_count: Option<u64>,
    url_link_clicks: Option<u64>,
    user_profile_clicks: Option<u64>,
    retweet_count: Option<u64>,
    reply_count: Option<u64>,
    like_count: Option<u64>,
}

#[derive(Debug, Deserialize, XData)]
pub struct PromotedMetrics {
    impression_count: Option<u64>,
    url_link_clicks: Option<u64>,
    user_profile_clicks: Option<u64>,
    retweet_count: Option<u64>,
    reply_count: Option<u64>,
    like_count: Option<u64>,
}

#[derive(Debug, Deserialize, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
pub enum ReplySettings {
    #[strum(serialize = "everyone")]
    Everyone,
    #[strum(serialize = "mentioned_users")]
    MentionedUsers,
    #[strum(serialize = "following")]
    Following,
}

#[derive(Debug, Deserialize)]
pub struct Includes {
    pub tweets: Option<Vec<Tweet>>,
    pub users: Option<Vec<User>>,
    pub places: Option<Vec<Place>>,
    pub media: Option<Vec<Media>>,
    pub polls: Option<Vec<Poll>>,
}

#[derive(Debug, Deserialize, IntoStaticStr, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "attachments")]
    Attachments,
    #[strum(serialize = "author_id")]
    AuthorId,
    #[strum(serialize = "context_annotations")]
    ContextAnnotations,
    #[strum(serialize = "conversation_id")]
    ConversationId,
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[strum(serialize = "edit_controls")]
    EditControls,
    #[strum(serialize = "entities")]
    Entities,
    #[strum(serialize = "geo")]
    Geo,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "in_reply_to_user_id")]
    InReplyToUserId,
    #[strum(serialize = "lang")]
    Lang,
    #[strum(serialize = "non_public_metrics")]
    NonPublicMetrics,
    #[strum(serialize = "public_metrics")]
    PublicMetrics,
    #[strum(serialize = "organic_metrics")]
    OrganicMetrics,
    #[strum(serialize = "promoted_metrics")]
    PromotedMetrics,
    #[strum(serialize = "possibly_sensitive")]
    PossiblySensitive,
    #[strum(serialize = "referenced_tweets")]
    ReferencedTweets,
    #[strum(serialize = "reply_settings")]
    ReplySettings,
    #[strum(serialize = "source")]
    Source,
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "withheld")]
    Withheld,
}

#[derive(Debug, Deserialize, XData)]
pub struct Tweet {
    id: Option<String>,
    text: Option<String>,
    created_at: Option<DateTime<Utc>>,
    author_id: Option<String>,
    edit_history_tweet_ids: Option<Vec<String>>,
    edit_controls: Option<EditControls>,
    conversation_id: Option<String>,
    note_tweet: Option<NoteTweet>,
    in_reply_to_user_id: Option<String>,
    referenced_tweets: Option<Vec<Tweet>>,
    attachments: Option<Vec<Attachment>>,
    geo: Option<Geo>,
    context_annotations: Option<Vec<ContextAnnotations>>,
    entities: Option<Entities>,
    withheld: Option<Withheld>,
    public_metrics: Option<PublicMetrics>,
    nonpublic_metrics: Option<NonpublicMetrics>,
    organic_metrics: Option<OrganicMetrics>,
    promoted_metrics: Option<PromotedMetrics>,
    possibly_sensitive: Option<bool>,
    lang: Option<String>,
    reply_settings: Option<ReplySettings>,
    source: Option<String>,
    includes: Option<Includes>,
}
