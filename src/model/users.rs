use chrono::{DateTime, Utc};
use getset::Getters;
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

#[derive(Debug, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Includes {
    #[strum(to_string = "tweets")]
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

pub type Users = Vec<User>;

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
