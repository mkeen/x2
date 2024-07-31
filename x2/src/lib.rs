// todo: possibly converge public and private prelude areas

pub(crate) mod _prelude {
    pub use crate::model::error::*;
    pub use derive_getters::Getters;
    pub use serde::Deserialize;
    pub use strum::{EnumCount, EnumIs, IntoStaticStr};
}

pub mod prelude {
    pub use crate::model::error::*;
    pub use derive_getters::Getters;
    pub use serde::Deserialize;
    pub use strum::{EnumCount, EnumIs, IntoStaticStr};
}

pub mod requests;
pub mod responses;

mod config;
pub mod model;

extern crate proc_macro;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_api_toml() {
//         println!("{:?}", config::get() );
//     }
// }
