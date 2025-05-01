use super::prelude::*;

pub type Response = super::Data;

impl<'a> super::Response<'a> for Response {
    type Request = requests::users::lookup::Request<'a>;
}
