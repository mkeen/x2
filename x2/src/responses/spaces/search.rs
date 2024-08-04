use super::prelude::*;

pub type Response = Data<Vec<Space>, Includes, 0>;

impl super::Response for Response {
    type Request = requests::spaces::search::Request;
}
