use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{
    entities::{DomainEntityInfo, Entities, Entity},
    media::Media,
    places::Place,
    polls::Poll,
    user::User,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextAnnotations {
    Domain(Option<Vec<DomainEntityInfo>>),
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplySettings {
    Everyone,
    MentionedUsers,
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

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            auth::{AppCredential, RequestCredential},
            error::XError,
        },
        requests::Request,
    };

    #[test]
    // todo: this is a temporary test. can make integration tests tho, just need to read keys from ENV, etc
    fn integration_usage_tweets_with_defaults() {
        let client_id = "gUJTmN2jcD7zOg2kFcbbS3fSp";
        let client_secret = "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon";

        let authentication: Result<RequestCredential, XError> = AppCredential::AppOnly {
            client_id,
            client_secret,
        }
        .try_into();

        if let Ok(auth) = authentication {
            let r = crate::requests::usage_tweets::Request::new(auth, None).request();
            //println!("{:?}", r);
            assert!(r.is_ok());
        } else {
            assert!(false);
        }
    }
}
