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

static DEFAULT_URL_PARAMS: [&str; 0] = [];
static URL_PARAM_FLAG: &str = "{}";
static URL_PARAM_NAME: &str = "Path";

#[derive(Debug, Deserialize, PartialEq, EnumProperty)]
#[serde(tag = "request")]
pub enum Endpoint {
    #[strum(props(Path = "oauth2/token"))]
    Authentication,
    #[strum(props(Path = "1.1/application/rate_limit_status.json"))]
    RateLimit,
    #[strum(props(Path = "2/usage/tweets", RateAppUnit = "50", RateAppMinute = "15"))]
    UsageTweets,
    #[strum(props(Path = "2/spaces/search", RateAppUnit = "50", RateAppMinute = "15"))]
    SpacesSearch,
    #[strum(props(
        Path = "2/users/by",
        RateAppUnit = "300",
        RateAppMinute = "15",
        RateUserUnit = "900",
        RateUserMinute = "15",
    ))]
    UserLookup,
    #[strum(props(Path = "2/users/{}/muting", RateUserUnit = "15", RateUserMinute = "15"))]
    UserMuting,
    #[strum(props(
        Path = "2/users/{}/muting/{}",
        RateUserUnit = "50",
        RateUserMinute = "15"
    ))]
    UserMutingManageUnmute,
    #[strum(props(Path = "2/users/{}/muting", RateUserUnit = "50", RateUserMinute = "15"))]
    UserMutingManageMute,
    #[strum(props(
        Path = "2/users/{}/blocking",
        RateUserUnit = "15",
        RateUserMinute = "15"
    ))]
    UserBlocking,
}

impl<'a> Endpoint {
    fn get_request_path(self) -> &'a str {
        self.get_str(URL_PARAM_NAME).unwrap()
    }

    fn replace_url_params(self, params: &[&str]) -> String {
        self.get_request_path()
            .split(URL_PARAM_FLAG)
            .enumerate()
            .map(|(i, s)| format!("{}{}", s, params.get(i).map_or("", |&x| x)))
            .collect::<String>()
    }

    pub fn url(self, params: Option<&[&str]>) -> Url {
        let params = params.unwrap_or(&DEFAULT_URL_PARAMS);
        Url::parse(&format!(
            "https://api.twitter.com/{}",
            self.replace_url_params(params)
        ))
        .expect("bad static url definition")
    }
}
