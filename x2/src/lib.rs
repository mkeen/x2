// todo: possibly converge public and private prelude areas

extern crate proc_macro;

pub(crate) mod _prelude {
    pub use super::API;
    pub use crate::model::error::*;
    pub use derive_getters::Getters;
    pub use serde::Deserialize;
    pub use strum::{self, EnumCount, EnumIs, EnumProperty, IntoStaticStr};
}

pub mod prelude {
    pub use crate::model::error::*;
    pub use derive_getters::Getters;
    pub use serde::Deserialize;
    pub use strum::{self, EnumCount, EnumIs, EnumProperty, IntoStaticStr};
}

use prelude::*;

mod config;
use config::*;

pub mod requests;
pub mod responses;

pub mod model;

use reqwest::Url;
use strum::EnumProperty;

pub(self) use requests::auth::Endpoint as AEndpoint;
//pub(self) use requests::compliance::Endpoint as CEndpoint;
//pub(self) use requests::list::Endpoint as LEndpoint;
//pub(self) use requests::messages::Endpoint as MEndpoint;
pub(self) use requests::spaces::Endpoint as SEndpoint;
//pub(self) use requests::trend::Endpoint as TDEndpoint;
//pub(self) use requests::tweet::Endpoint as TEndpoint;
//pub(self) use requests::usage::Endpoint as USEndpoint;
pub(self) use requests::users::Endpoint as UEndpoint;

#[derive(Debug, EnumProperty)]
pub enum API {
    Auth(AEndpoint),
    //Tweet(TEndpoint),
    //Usage(USEndpoint),
    //Trend(TDEndpoint),
    Space(SEndpoint),
    User(UEndpoint),
    //Message(MEndpoint),
    //List(LEndpoint),
    //Compliance(CEndpoint),
    //#[strum(props(Path = "1.1/application/rate_limit_status.json"))]
    //RateLimit,
    //#[strum(props(Path = "2/usage/tweets", RateAppUnit = "50", RateAppMinute = "15"))]
    //UsageTweets,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_api_toml() {
//         println!("{:?}", config::get() );
//     }
// }
