use super::prelude::*;

pub type Response = Data<Vec<Tweet>, Includes, 1>;

impl super::Response for Response {
    type Request = requests::tweets::search::Request;
}
