use super::prelude::*;

pub type Response = Data<Vec<User>, Includes>;

impl<'a> super::super::Response<'a> for Response {
    type Request = requests::users::muting::lookup::Request<'a>;
}
