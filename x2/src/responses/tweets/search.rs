use super::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Data {
    data: Vec<Tweet>,
    includes: Option<Includes>,
    meta: Option<Meta>,
}

impl Data {}

pub type Response = Data;

impl<'a> super::Response<'a> for Response {
    type Request = requests::tweets::search::Request<'a>;
}

impl<'a> super::Paginated<'a> for Response {
    fn meta(&self) -> &Option<Meta> {
        &self.meta
    }

    fn next_page(&self) -> Option<&String> {
        match self.meta() {
            Some(meta) => meta.next_token(),
            None => None,
        }
    }
}
