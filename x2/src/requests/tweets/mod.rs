pub(crate) mod prelude {
    pub(crate) use super::super::Endpoint as EndpointTrait;
    pub(crate) use super::super::{client, prelude::*};
    pub(crate) use crate::model::media::Field as MediaField;
    pub(crate) use crate::model::places::Field as PlaceField;
    pub(crate) use crate::model::polls::Field as PollField;
    pub(crate) use crate::model::tweets::Field;
    pub(crate) use crate::model::users::Field as UserField;
}

pub(crate) use super::Authorized;
pub(crate) use super::Request;

pub mod lookup;
pub mod search;
pub mod timelines;

use prelude::*;

#[derive(Debug, EnumProperty, UrlEndpoint)]
pub enum Endpoint {
    #[strum(props(
        Path = "2/users/tweets",
        RateUserUnit = "900",
        RateUserMinute = "15",
        RateAppUnit = "900",
        RateAppMinute = "15"
    ))]
    Lookup,
    #[strum(props(
        Path = "2/users/{}/tweets",
        RateUserUnit = "900",
        RateUserMinute = "15",
        RateAppUnit = "1500",
        RateAppMinute = "15"
    ))]
    Timelines,
    #[strum(props(
        Path = "2/tweets/search/recent",
        RateUserUnit = "180",
        RateUserMinute = "15",
        RateAppUnit = "450",
        RateAppMinute = "15"
    ))]
    Search,
}
