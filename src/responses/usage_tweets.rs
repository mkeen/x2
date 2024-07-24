use crate::model::{error::XError, users::Includes};
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub use super::Response as ResponseTrait;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub project_id: Option<String>,
    pub project_cap: Option<String>,
    pub project_usage: Option<String>,
    pub cap_reset_day: Option<u32>,
    pub daily_project_usage: Option<DailyProjectUsage>,
    pub daily_client_app_usage: Option<DailyClientAppUsage>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "data")]
pub struct Response {
    data: Data,
    includes: Option<Includes>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Usage {
    pub date: DateTime<Utc>,
    pub usage: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DailyProjectUsage {
    pub project_id: String,
    pub usage: Vec<Usage>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DailyClientAppUsage {
    pub usage: Vec<Usage>,
}

impl ResponseTrait for Response {
    type Response = Response;

    fn try_into_from_bytes(bytes: &[u8]) -> Result<Response, XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}
