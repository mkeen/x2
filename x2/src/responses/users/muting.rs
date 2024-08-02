use super::prelude::*;

#[derive(Debug, Deserialize)]
pub struct MutingStatus {
    pub muting: bool,
}

pub type Response = Pattern<SimpleData<MutingStatus>>;

impl super::super::Response for Response {}
