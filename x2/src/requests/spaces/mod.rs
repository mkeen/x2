pub mod prelude {
    pub(crate) use super::super::{prelude::*, RequestBuilder};
    pub(crate) use crate::model::spaces::*;
}

use prelude::*;

pub(crate) use super::Request;

pub mod search;
