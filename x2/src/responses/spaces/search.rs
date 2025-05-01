use super::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub data: Vec<Space>,
    pub includes: Option<Includes>,
    pub meta: Option<super::super::Meta>,
}

pub type Response = Data;

impl<'a> super::Response<'a> for Response {
    type Request = requests::spaces::search::Request<'a>;
}
