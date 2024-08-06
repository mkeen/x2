use super::prelude::*;

pub type Response = Data<Vec<Space>, Includes, 0>;

impl<'a> super::Response<'a> for Response {
    type Request = requests::spaces::search::Request<'a>;
}
