pub(crate) mod prelude {
    pub(crate) use super::super::prelude::*;
}

use prelude::EnumProperty;

pub(crate) use super::Authorized;
pub(crate) use super::Request;

#[derive(Debug, EnumProperty)]
pub enum Endpoint {
    #[strum(props(
        Path = "2/users/{}/bookmarks",
        RateUserUnit = "50",
        RateUserMinute = "15"
    ))]
    ManageAddBookmark,
    #[strum(props(
        Path = "2/users/{}/bookmarks/{}",
        RateUserUnit = "50",
        RateUserMinute = "15"
    ))]
    ManageRemoveBookmark,
    #[strum(props(
        Path = "2/users/{}/bookmarks",
        RateUserUnit = "180",
        RateUserMinute = "15"
    ))]
    Lookup,
}

impl super::super::Endpoint for Endpoint {}

pub mod lookup;
pub mod manage;
