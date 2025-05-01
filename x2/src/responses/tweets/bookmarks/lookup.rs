use crate::requests;

use super::prelude::*;

pub type Response = Data<Vec<Tweet>, Includes>;

impl<'a> super::super::Response<'a> for Response {
    type Request = requests::tweets::bookmarks::lookup::Request<'a>;
}
