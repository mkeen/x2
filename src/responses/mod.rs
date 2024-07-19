use std::io::Bytes;

use crate::model::auth::RequestCredential;

pub trait Response<'a> {}

pub mod auth;
pub mod rate_limit;
