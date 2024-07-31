pub(crate) mod prelude {
    pub use crate::model::error::*;
    pub use derive_getters::Getters;
    pub use serde::Deserialize;
    pub use strum::{AsRefStr, EnumCount, EnumIs};
}

pub mod requests;
pub mod responses;

mod config;
mod model;

extern crate proc_macro;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_api_toml() {
//         println!("{:?}", config::get() );
//     }
// }
