use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde::Deserialize;
use strum::{AsRefStr, EnumCount};

use super::{entities::Entities, tweets::Tweet, withheld::Withheld};

#[derive(Debug, AsRefStr, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifiedType {
    #[strum(serialize = "blue")]
    Blue,
    #[strum(serialize = "business")]
    Business,
    #[strum(serialize = "government")]
    Government,
    #[strum(serialize = "none")]
    None,
}

#[derive(Debug, Deserialize)]
pub struct PublicMetrics {
    pub followers_count: Option<String>,
    pub following_count: Option<String>,
    pub tweet_count: Option<String>,
    pub listed_count: Option<String>,
}

#[derive(Debug, Deserialize, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum Includes {
    #[strum(serialize = "tweets")]
    Tweets(Vec<Tweet>),
}

#[derive(Deserialize, AsRefStr, EnumCount)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[strum(serialize = "description")]
    Description,
    #[strum(serialize = "entities")]
    Entities,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "most_recent_tweet_id")]
    MostRecentTweetId,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "pinned_tweet_id")]
    PinnedTweetId,
    #[strum(serialize = "profile_image_url")]
    ProfileImageUrl,
    #[strum(serialize = "protected")]
    Protected,
    #[strum(serialize = "public_metrics")]
    PublicMetrics,
    #[strum(serialize = "url")]
    Url,
    #[strum(serialize = "username")]
    Username,
    #[strum(serialize = "verified")]
    Verified,
    #[strum(serialize = "verified_type")]
    VerifiedType,
    #[strum(serialize = "withheld")]
    Withheld,
}

#[derive(Debug, Deserialize, Getters)]
pub struct User {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
    created_at: Option<DateTime<Utc>>,
    most_recent_tweet_id: Option<String>,
    protected: Option<bool>,
    withheld: Option<Withheld>,
    location: Option<String>,
    url: Option<String>,
    description: Option<String>,
    verified: Option<bool>,
    verified_type: Option<VerifiedType>,
    entities: Option<Entities>,
    profile_image_url: Option<String>,
    public_metrics: Option<PublicMetrics>,
    pinned_tweet_id: Option<String>,
}
