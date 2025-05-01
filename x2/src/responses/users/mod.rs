pub(crate) use super::prelude as _prelude;

use prelude::{Deserialize, Includes, User};

mod prelude {
    pub use super::_prelude::*;
    pub use crate::model::users::*;
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub data: Vec<User>,
    pub includes: Option<Includes>,
    pub meta: Option<super::Meta>,
}

pub(crate) use super::Response;

pub mod blocking;
pub mod lookup;
pub mod muting;
