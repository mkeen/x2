mod prelude {
    pub use super::super::prelude::*;
}

use prelude::{Deserialize, Includes, User};

pub(crate) use super::Response;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub data: Vec<User>,
    pub includes: Option<Includes>,
    pub meta: Option<super::super::Meta>,
}

pub mod lookup;
