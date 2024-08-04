use super::prelude::*;

pub type Response = Data<Vec<Tweet>, Includes, 0>;

impl super::Response for Response {
    type Request = requests::tweets::lookup::Request;
}
