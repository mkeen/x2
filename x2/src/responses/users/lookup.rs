use super::prelude::*;

pub type Response = Data<Vec<User>, Includes, 3>;

impl<'a> super::Response<'a> for Response {
    type Request = requests::users::lookup::Request<'a>;
}
