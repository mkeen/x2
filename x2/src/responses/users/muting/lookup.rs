use super::prelude::*;

pub type Response = Pattern<Data<Vec<User>, Includes, 0>>;

impl<'a> super::super::Response<'a> for Response {
    type Request = requests::users::muting::lookup::Request<'a>;
}
