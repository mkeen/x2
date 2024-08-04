pub(crate) mod prelude {
    pub(crate) use super::super::Endpoint as EndpointTrait;
    pub(crate) use super::super::{client, prelude::*};
    pub(crate) use crate::model::tweets::Field as TweetField;
    pub(crate) use crate::model::users::Field;
}

pub(crate) use super::Authorized;
pub(crate) use super::Request;

pub mod blocking;
//pub mod bookmarks;
//pub mod followers;
pub mod lookup;
pub mod muting;
//pub mod search;

use prelude::*;

#[derive(Debug, EnumProperty, UrlEndpoint)]
pub enum Endpoint {
    #[strum(props(
        Path = "2/users/{}/blocking",
        RateUserUnit = "15",
        RateUserMinute = "15"
    ))]
    Blocking,
    #[strum(props(
        Path = "2/users/{}/followers",
        RateUserUnit = "15",
        RateUserMinute = "15"
    ))]
    Followers,
    #[strum(props(
        Path = "2/users/by",
        RateAppUnit = "300",
        RateAppMinute = "15",
        RateUserUnit = "900",
        RateUserMinute = "15",
    ))]
    Lookup,
    //Muting(muting::Endpoint),
    #[strum(props(Path = "2/users/search", RateUserUnit = "15", RateUserMinute = "15"))]
    Search,
}
