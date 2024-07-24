use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum::Display;

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

#[derive(Debug, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ContextAnnotations {
    #[strum(to_string = "domain")]
    Domain(Option<Vec<DomainEntityInfo>>),
    #[strum(to_string = "entity")]
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

#[derive(Debug, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ReplySettings {
    #[strum(to_string = "everyone")]
    Everyone,
    #[strum(to_string = "mentioned_users")]
    MentionedUsers,
    #[strum(to_string = "following")]
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

#[derive(Debug, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(to_string = "attachments")]
    Attachments,
    #[strum(to_string = "author_id")]
    AuthorId,
    #[strum(to_string = "context_annotations")]
    ContextAnnotations,
    #[strum(to_string = "conversation_id")]
    ConversationId,
    #[strum(to_string = "created_at")]
    CreatedAt,
    #[strum(to_string = "edit_controls")]
    EditControls,
    #[strum(to_string = "entities")]
    Entities,
    #[strum(to_string = "geo")]
    Geo,
    #[strum(to_string = "id")]
    Id,
    #[strum(to_string = "in_reply_to_user_id")]
    InReplyToUserId,
    #[strum(to_string = "lang")]
    Lang,
    #[strum(to_string = "non_public_metrics")]
    NonPublicMetrics,
    #[strum(to_string = "public_metrics")]
    PublicMetrics,
    #[strum(to_string = "organic_metrics")]
    OrganicMetrics,
    #[strum(to_string = "promoted_metrics")]
    PromotedMetrics,
    #[strum(to_string = "possibly_sensitive")]
    PossiblySensitive,
    #[strum(to_string = "referenced_tweets")]
    ReferencedTweets,
    #[strum(to_string = "reply_settings")]
    ReplySettings,
    #[strum(to_string = "source")]
    Source,
    #[strum(to_string = "text")]
    Text,
    #[strum(to_string = "withheld")]
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
