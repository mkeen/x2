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
    pub use super::{model, requests, responses};
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

pub(crate) mod test_util {
    use crate::{
        model::auth::{Method, RequestCredential},
        requests::tweets::prelude::Context,
    };

    pub fn oauth1_credentials() -> Context {
        let consumer_id = "c2HAMlWTX2m3cVgNgA0oqLRqH".to_string();
        let consumer_secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2".to_string();

        let oauth2_client_id = "TV9xZXRVVVN0STIwSkcwck9WS2w6MTpjaQ".to_string();
        let oauth2_client_secret = "gZHqK9YQZyrH7x7P9Yg5kxdE3j8_yDQopjBxXIptw-4b2TIM4_".to_string();

        let user_id = "1444148135954108418-TSUe6cI1lpIddYScxSKIlmbfq71kyL".to_string();
        let user_secret = "vupepUIBVJl08dhMdlHuNTyRWaWUVPenrPpSl1E4EqWb6".to_string();

        Context::Request(RequestCredential::OAuth10AConsumer {
            consumer_id,
            consumer_secret,
            user_id,
            user_secret,
        })
    }

    pub fn app_only_unauthed_credentials() -> Context {
        let consumer_id = "c2HAMlWTX2m3cVgNgA0oqLRqH".to_string();
        let consumer_secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2".to_string();

        let oauth2_client_id = "TV9xZXRVVVN0STIwSkcwck9WS2w6MTpjaQ".to_string();
        let oauth2_client_secret = "gZHqK9YQZyrH7x7P9Yg5kxdE3j8_yDQopjBxXIptw-4b2TIM4_".to_string();

        let user_id = "1444148135954108418-TSUe6cI1lpIddYScxSKIlmbfq71kyL".to_string();
        let user_secret = "vupepUIBVJl08dhMdlHuNTyRWaWUVPenrPpSl1E4EqWb6".to_string();

        Context::Caller(Method::AppOnly {
            id: consumer_id,
            secret: consumer_secret,
        })
    }
}
