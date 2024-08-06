use super::prelude::*;

#[derive(Debug, Deserialize)]
pub struct MutingStatus {
    pub muting: bool,
}

pub type Response = Pattern<SimpleData<MutingStatus>>;

impl MutingStatus {}

impl<'a> super::super::Response<'a> for Response {
    type Request = requests::users::muting::manage::Request<'a>;
}
