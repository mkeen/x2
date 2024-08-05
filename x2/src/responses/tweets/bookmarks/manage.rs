use super::prelude::*;

#[derive(Debug, Deserialize)]
pub struct BookmarkedStatus {
    pub bookmarked: bool,
}

pub type Response = Pattern<SimpleData<BookmarkedStatus>>;

impl BookmarkedStatus {}

impl super::super::Response for Response {
    type Request = requests::tweets::bookmarks::manage::Request;
}
