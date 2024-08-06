use super::prelude::*;

#[derive(Debug, Deserialize)]
pub struct BookmarkedStatus {
    pub bookmarked: bool,
}

pub type Response = Pattern<SimpleData<BookmarkedStatus>>;

impl BookmarkedStatus {}

impl<'a> super::super::Response<'a> for Response {
    type Request = requests::tweets::bookmarks::manage::Request<'a>;
}
