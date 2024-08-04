use super::prelude::*;

pub type Response = Data<Vec<User>, Includes, 3>;

impl super::Response for Response {
    type Request = requests::users::lookup::Request;
}
