//! # X2
//!
//! [![Build Status](https://travis-ci.org/Peternator7/strum.svg?branch=master)](https://travis-ci.org/Peternator7/strum)
//! [![Latest Version](https://img.shields.io/crates/v/strum.svg)](https://crates.io/crates/strum)
//! [![Rust Documentation](https://docs.rs/strum/badge.svg)](https://docs.rs/strum)
//!
//! X2 is a library for interacting with the X v2 APIs
//!
//! The full version of the README can be found on [GitHub](https://github.com/mkeen/X2).
//!
//! # Including X2 in Your Project
//!
//! ```toml
//! [dependencies]
//! x2 = "0.1"
//! ```
//!

pub(crate) mod _prelude {
    pub(crate) use super::model;
    pub use crate::model::error::*;
    pub use serde::Deserialize;
    pub use strum::{self, EnumCount, EnumDiscriminants, EnumIs, EnumProperty, IntoStaticStr};
}

pub mod prelude {
    pub use crate::model::error::*;
    pub use serde::Deserialize;
    pub use strum::{self, EnumCount, EnumDiscriminants, EnumIs, EnumProperty, IntoStaticStr};
}

mod config;
use config::*;

pub mod requests;
pub mod responses;

pub mod model;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_api_toml() {
//         println!("{:?}", config::get() );
//     }
// }
