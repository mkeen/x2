use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum::{Display, EnumString};

use super::user::User;

#[derive(Debug, Deserialize, EnumString)]
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
    pub users: Option<Vec<User>>,
}

#[derive(Deserialize, Display, EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    #[strum(to_string = "invited_user_ids")]
    InvitedUserIds,
    #[strum(to_string = "speaker_ids")]
    SpeakerIds,
    #[strum(to_string = "creator_id")]
    CreatorId,
    #[strum(to_string = "host_ids")]
    HostIds,
    #[strum(to_string = "topic_ids")]
    TopicIds,
}

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
    pub id: Option<String>,
    pub host_ids: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub creator_id: Option<String>,
    pub lang: Option<String>,
    pub is_ticketed: Option<bool>,
    pub invited_user_ids: Option<Vec<String>>,
    pub participant_count: Option<u64>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub speaker_ids: Option<Vec<String>>,
    pub state: Option<State>,
    pub subscriber_count: Option<u64>,
    pub topic_ids: Option<Vec<String>>,
    pub topics: Option<Vec<Topic>>,
    pub title: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            auth::{AppCredential, RequestCredential},
            error::XError,
            spaces::{Expansion, Field, State},
        },
        requests::Request,
    };

    #[test]
    fn integration_spaces_search_with_defaults() {
        let client_id = "gUJTmN2jcD7zOg2kFcbbS3fSp";
        let client_secret = "8tWsU562uAzSFaCP7860rGHd0yldWgDJGwwvlyrugqoGBB8qon";

        let authentication: Result<RequestCredential, XError> = AppCredential::AppOnly {
            client_id,
            client_secret,
        }
        .try_into();

        if let Ok(auth) = authentication {
            let r = crate::requests::spaces_search::Request::new(
                auth,
                "cardano",
                Some(vec![
                    Expansion::HostIds,
                    Expansion::SpeakerIds,
                    Expansion::CreatorId,
                    Expansion::TopicIds,
                    Expansion::InvitedUserIds,
                ]),
                Some(vec![
                    Field::Title,
                    Field::HostIds,
                    Field::CreatedAt,
                    Field::CreatorId,
                    Field::Id,
                    Field::Lang,
                    Field::StartedAt,
                    Field::EndedAt,
                    Field::SpeakerIds,
                    Field::InvitedUserIds,
                    Field::ParticipantCount,
                    Field::TopicIds,
                    Field::State,
                    Field::IsTicketed,
                    Field::ScheduledStart,
                ]),
                None,
                None,
                Some(State::Live),
            )
            .request();
            println!("{:?}", r);
            assert!(r.is_ok());
        } else {
            assert!(false);
        }
    }
}
