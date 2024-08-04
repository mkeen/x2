use super::prelude::*;

pub type Response = Data<Vec<Space>, Includes, 1>;

impl super::Response for Response {
    type Request = requests::spaces::lookup::Request;
}
