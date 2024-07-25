use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PollOption {
    pub position: Option<usize>,
    pub label: Option<String>,
    pub votes: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Poll {
    pub id: Option<String>,
    pub options: Option<Vec<PollOption>>,
    pub duration_minutes: Option<u64>,
    pub end_datetime: Option<DateTime<Utc>>,
    pub voting_status: Option<String>,
}
