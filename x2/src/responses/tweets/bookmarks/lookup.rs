use crate::requests;

use super::prelude::*;

pub type Response = Pattern<Data<Vec<Tweet>, Includes, 2>>;

impl<'a> super::super::Response<'a> for Response {
    type Request = requests::tweets::bookmarks::lookup::Request<'a>;
}
