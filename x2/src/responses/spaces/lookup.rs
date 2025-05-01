use super::prelude::*;

pub type Response = Data<Vec<Space>, Includes>;

impl<'a> super::Response<'a> for Response {
    type Request = requests::spaces::lookup::Request<'a>;
}
