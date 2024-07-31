use reqwest::Url;
use serde::Deserialize;
use std::sync::OnceLock;
use strum::EnumProperty;

#[derive(Debug, Deserialize)]
struct EndpointsConfig {
    constants: Constants,
}

impl EndpointsConfig {}

#[derive(Debug, Deserialize)]
struct Constants {
    base_url: String,
}

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
}

impl<'a> Endpoint {
    pub fn url(self) -> Url {
        Url::parse(&format!(
            "https://{}/{}",
            "api.twitter.com",
            self.get_str("Path").unwrap()
        ))
        .expect("lib error, could not find url for request type")
    }
}
