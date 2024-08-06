use super::prelude::*;

pub type Response = Data<Vec<Tweet>, Includes, 1>;

impl<'a> super::Response<'a> for Response {
    type Request = requests::tweets::search::Request<'a>;
}
