use super::prelude::{AsRefStr, Deserialize, EnumCount};

use super::users::User;
use chrono::{self, DateTime, Utc};

#[derive(Debug, Deserialize, AsRefStr, Default)]
#[serde(rename_all = "snake_case")]
pub enum State {
    #[strum(serialize = "live")]
    Live,
    #[strum(serialize = "scheduled")]
    Scheduled,
    #[strum(serialize = "all")]
    #[default]
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

#[derive(Debug, Deserialize, AsRefStr, PartialEq, EnumCount)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "host_ids")]
    HostIds,
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[strum(serialize = "creator_id")]
    CreatorId,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "lang")]
    Lang,
    #[strum(serialize = "invited_user_ids")]
    InvitedUserIds,
    #[strum(serialize = "participant_count")]
    ParticipantCount,
    #[strum(serialize = "speaker_ids")]
    SpeakerIds,
    #[strum(serialize = "started_at")]
    StartedAt,
    #[strum(serialize = "ended_at")]
    EndedAt,
    #[strum(serialize = "subscriber_count")]
    SubscriberCount,
    #[strum(serialize = "topic_ids")]
    TopicIds,
    #[strum(serialize = "state")]
    State,
    #[strum(serialize = "title")]
    Title,
    #[strum(serialize = "updated_at")]
    UpdatedAt,
    #[strum(serialize = "scheduled_start")]
    ScheduledStart,
    #[strum(serialize = "is_ticketed")]
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
