use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum::{Display, EnumString};

use super::{entities::Entities, tweets::Tweet, withheld::Withheld};

#[derive(Debug, EnumString, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum VerifiedType {
    #[strum(to_string = "blue")]
    Blue,
    #[strum(to_string = "business")]
    Business,
    #[strum(to_string = "government")]
    Government,
    #[strum(to_string = "none")]
    None,
}

#[derive(Debug, Deserialize)]
pub struct PublicMetrics {
    pub followers_count: Option<String>,
    pub following_count: Option<String>,
    pub tweet_count: Option<String>,
    pub listed_count: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Includes {
    Tweets(Vec<Tweet>),
}

#[derive(Deserialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(to_string = "created_at")]
    CreatedAt,
    #[strum(to_string = "description")]
    Description,
    #[strum(to_string = "entities")]
    Entities,
    #[strum(to_string = "id")]
    Id,
    #[strum(to_string = "location")]
    Location,
    #[strum(to_string = "most_recent_tweet_id")]
    MostRecentTweetId,
    #[strum(to_string = "name")]
    Name,
    #[strum(to_string = "pinned_tweet_id")]
    PinnedTweetId,
    #[strum(to_string = "profile_image_url")]
    ProfileImageUrl,
    #[strum(to_string = "protected")]
    Protected,
    #[strum(to_string = "public_metrics")]
    PublicMetrics,
    #[strum(to_string = "url")]
    Url,
    #[strum(to_string = "username")]
    Username,
    #[strum(to_string = "verified")]
    Verified,
    #[strum(to_string = "verified_type")]
    VerifiedType,
    #[strum(to_string = "withheld")]
    Withheld,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub most_recent_tweet_id: Option<String>,
    pub protected: Option<bool>,
    pub withheld: Option<Withheld>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub verified: Option<bool>,
    pub verified_type: Option<VerifiedType>,
    pub entities: Option<Entities>,
    pub profile_image_url: Option<String>,
    pub public_metrics: Option<PublicMetrics>,
    pub pinned_tweet_id: Option<String>,
    pub includes: Option<Includes>,
}
