use reqwest::Url;
use serde::Deserialize;
use std::sync::OnceLock;
use strum::EnumProperty;

use crate::prelude::LibError;

#[derive(Debug, Deserialize)]
struct EndpointsConfig {
    constants: Constants,
}

impl EndpointsConfig {}

#[derive(Debug, Deserialize)]
struct Constants {
    base_url: String,
}

pub(super) static DEFAULT_URL_PARAMS: [&str; 0] = [];
pub(super) static URL_PARAM_FLAG: &str = "{}";
pub(super) static URL_PARAM_NAME: &str = "Path";
