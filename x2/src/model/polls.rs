use super::prelude::{Deserialize, EnumCount, IntoStaticStr};

use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct PollOption {
    pub position: Option<usize>,
    pub label: Option<String>,
    pub votes: Option<u64>,
}

#[derive(Deserialize, IntoStaticStr, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "duration_minutes")]
    DurationMinutes,
    #[strum(serialize = "end_datetime")]
    EndDatetime,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "options")]
    Options,
    #[strum(serialize = "voting_status")]
    VotingStatus,
}

#[derive(Debug, Deserialize)]
pub struct Poll {
    pub id: Option<String>,
    pub options: Option<Vec<PollOption>>,
    pub duration_minutes: Option<u64>,
    pub end_datetime: Option<DateTime<Utc>>,
    pub voting_status: Option<String>,
}
