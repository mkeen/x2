pub mod prelude {
    pub(crate) use super::super::prelude::*;
    pub(crate) use super::super::Endpoint as EndpointTrait;
    pub(crate) use crate::model::spaces::*;
}

use prelude::*;

pub(crate) use super::Authorized;
pub(crate) use super::Request;

pub mod search;

#[derive(Debug, EnumProperty)]
pub enum Endpoint {
    #[strum(props(Path = "2/spaces/search", RateAppUnit = "50", RateAppMinute = "15"))]
    Search,
}

impl super::Endpoint for Endpoint {}
