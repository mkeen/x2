pub(crate) use super::prelude as _prelude;

pub const R_ID: u8 = 0;

mod prelude {
    pub use super::super::{Data, Meta, SimpleData};
    pub use super::_prelude::*;
    pub use crate::model::users::*;
}

use prelude::*;

pub type Response = Pattern<Data<Vec<User>, Includes>>;

impl super::Response for Response {
    // User Basic List of Users Handler
}

pub mod muting;
