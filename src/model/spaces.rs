use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum::{Display, EnumString};

use super::users::User;

#[derive(Debug, Deserialize, EnumString, Display)]
#[serde(rename_all = "snake_case")]
pub enum State {
    #[strum(serialize = "live")]
    Live,
    #[strum(to_string = "scheduled")]
    Scheduled,
    #[strum(to_string = "all")]
    All,
}

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Includes {
    pub users: Vec<User>,
}

// #[derive(Deserialize, Display, EnumString, PartialEq)]
// #[serde(rename_all = "snake_case")]
// pub enum Expansion {
//     #[strum(to_string = "invited_user_ids")]
//     InvitedUserIds,
//     #[strum(to_string = "speaker_ids")]
//     SpeakerIds,
//     #[strum(to_string = "creator_id")]
//     CreatorId,
//     #[strum(to_string = "host_ids")]
//     HostIds,
//     #[strum(to_string = "topic_ids")]
//     TopicIds,
// }

#[derive(Debug, Display, Deserialize, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(to_string = "host_ids")]
    HostIds,
    #[strum(to_string = "created_at")]
    CreatedAt,
    #[strum(to_string = "creator_id")]
    CreatorId,
    #[strum(to_string = "id")]
    Id,
    #[strum(to_string = "lang")]
    Lang,
    #[strum(to_string = "invited_user_ids")]
    InvitedUserIds,
    #[strum(to_string = "participant_count")]
    ParticipantCount,
    #[strum(to_string = "speaker_ids")]
    SpeakerIds,
    #[strum(to_string = "started_at")]
    StartedAt,
    #[strum(to_string = "ended_at")]
    EndedAt,
    #[strum(to_string = "subscriber_count")]
    SubscriberCount,
    #[strum(to_string = "topic_ids")]
    TopicIds,
    #[strum(to_string = "state")]
    State,
    #[strum(to_string = "title")]
    Title,
    #[strum(to_string = "updated_at")]
    UpdatedAt,
    #[strum(to_string = "scheduled_start")]
    ScheduledStart,
    #[strum(to_string = "is_ticketed")]
    IsTicketed,
}

#[derive(Debug, Deserialize)]
pub struct Space {
    id: Option<String>,
    host_ids: Option<Vec<String>>,
    created_at: Option<DateTime<Utc>>,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
    creator_id: Option<String>,
    lang: Option<String>,
    is_ticketed: Option<bool>,
    invited_user_ids: Option<Vec<String>>,
    participant_count: Option<u64>,
    scheduled_start: Option<DateTime<Utc>>,
    speaker_ids: Option<Vec<String>>,
    state: Option<State>,
    subscriber_count: Option<u64>,
    topic_ids: Option<Vec<String>>,
    topics: Option<Vec<Topic>>,
    title: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    includes: Option<Includes>,
}
