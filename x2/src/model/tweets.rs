use super::prelude::{Deserialize, EnumCount, IntoStaticStr};

use chrono::{DateTime, Utc};

use super::{
    entities::{DomainEntityInfo, Entities, Entity},
    media::Media,
    places::Place,
    polls::Poll,
    users::User,
    withheld::Withheld,
};

#[derive(Debug, Deserialize)]
pub struct EditControls {
    pub is_edit_eligible: Option<bool>,
    pub editable_until: Option<DateTime<Utc>>,
    pub edits_remaining: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct NoteTweet {
    pub text: Option<String>,
    pub entities: Option<Vec<Entities>>,
}

#[derive(Debug, Deserialize)]
pub struct Attachment {
    pub media_keys: Option<Vec<String>>,
    pub poll_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub coordinates: Option<(f32, f32)>,
    pub place_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Geo {
    pub coordinates: Option<Coordinates>,
}

#[derive(Debug, Deserialize, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
pub enum ContextAnnotations {
    #[strum(serialize = "domain")]
    Domain(Option<Vec<DomainEntityInfo>>),
    #[strum(serialize = "entity")]
    Entity(Option<Vec<Entity>>),
}

#[derive(Debug, Deserialize)]
pub struct PublicMetrics {
    pub retweet_count: Option<u64>,
    pub reply_count: Option<u64>,
    pub like_count: Option<u64>,
    pub quote_count: Option<u64>,
    pub impression_count: Option<u64>,
    pub bookmark_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct NonpublicMetrics {
    pub impression_count: Option<u64>,
    pub url_link_clicks: Option<u64>,
    pub user_profile_clicks: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct OrganicMetrics {
    pub impression_count: Option<u64>,
    pub url_link_clicks: Option<u64>,
    pub user_profile_clicks: Option<u64>,
    pub retweet_count: Option<u64>,
    pub reply_count: Option<u64>,
    pub like_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct PromotedMetrics {
    pub impression_count: Option<u64>,
    pub url_link_clicks: Option<u64>,
    pub user_profile_clicks: Option<u64>,
    pub retweet_count: Option<u64>,
    pub reply_count: Option<u64>,
    pub like_count: Option<u64>,
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

#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub id: Option<String>,
    pub text: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub author_id: Option<String>,
    pub edit_history_tweet_ids: Option<Vec<String>>,
    pub edit_controls: Option<EditControls>,
    pub conversation_id: Option<String>,
    pub note_tweet: Option<NoteTweet>,
    pub in_reply_to_user_id: Option<String>,
    pub referenced_tweets: Option<Vec<Tweet>>,
    pub attachments: Option<Vec<Attachment>>,
    pub geo: Option<Geo>,
    pub context_annotations: Option<Vec<ContextAnnotations>>,
    pub entities: Option<Entities>,
    pub withheld: Option<Withheld>,
    pub public_metrics: Option<PublicMetrics>,
    pub nonpublic_metrics: Option<NonpublicMetrics>,
    pub organic_metrics: Option<OrganicMetrics>,
    pub promoted_metrics: Option<PromotedMetrics>,
    pub possibly_sensitive: Option<bool>,
    pub lang: Option<String>,
    pub reply_settings: Option<ReplySettings>,
    pub source: Option<String>,
    pub includes: Option<Includes>,
}
