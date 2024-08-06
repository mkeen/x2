use crate::requests;

use super::prelude::*;

pub type Response = Pattern<Data<Vec<User>, Includes, 1>>;

impl<'a> super::Response<'a> for Response {
    type Request = requests::users::blocking::Request<'a>;
}
