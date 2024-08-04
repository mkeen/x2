use super::prelude::*;

pub type Response = Pattern<Data<Vec<User>, Includes, 0>>;

impl super::super::Response for Response {
    type Request = requests::users::muting::lookup::Request;
}
