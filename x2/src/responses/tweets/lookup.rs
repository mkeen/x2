use super::prelude::*;

pub type Response = Data<Vec<Tweet>, Includes, 0>;

impl<'a> super::Response<'a> for Response {
    type Request = requests::tweets::lookup::Request<'a>;
}
