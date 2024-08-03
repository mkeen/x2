pub mod prelude {
    pub use super::super::prelude::*;
}

use prelude::*;

pub(crate) use super::Authorized;
pub(crate) use super::Request;

#[derive(Debug, EnumProperty)]
pub enum Endpoint {
    #[strum(props(Path = "2/users/{}/muting", RateUserUnit = "15", RateUserMinute = "15"))]
    Lookup,
    #[strum(props(
        Path = "2/users/{}/muting/{}",
        RateUserUnit = "50",
        RateUserMinute = "15"
    ))]
    Unmute,
    #[strum(props(Path = "2/users/{}/muting", RateUserUnit = "50", RateUserMinute = "15"))]
    Mute,
}

impl super::super::Endpoint for Endpoint {}

pub mod lookup;
pub mod manage;
