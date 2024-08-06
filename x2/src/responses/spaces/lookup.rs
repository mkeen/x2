use super::prelude::*;

pub type Response = Data<Vec<Space>, Includes, 1>;

impl<'a> super::Response<'a> for Response {
    type Request = requests::spaces::lookup::Request<'a>;
}
