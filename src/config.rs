use reqwest::Url;
use serde::Deserialize;
use std::sync::OnceLock;
use strum::EnumProperty;
use toml;

const RAW: &str = include_str!("./../api.toml");

#[derive(Debug, Deserialize)]
pub struct EndpointsConfig {
    constants: Constants,
    endpoints: Vec<Endpoint>,
}

impl EndpointsConfig {}

#[derive(Debug, Deserialize)]
pub struct Constants {
    version: String,
    base_url: String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct EndpointData {
    path: String,
}

#[derive(Debug, Deserialize, PartialEq, EnumProperty)]
#[serde(tag = "request")]
pub enum Endpoint {
    #[strum(props(Path = "oauth2/token"))]
    Authentication,
    #[strum(props(Path = "1.1/application/rate_limit_status.json"))]
    RateLimit,
}

impl Endpoint {
    pub fn url(self) -> Url {
        let config = get();

        Url::parse(&format!(
            "https://{}/{}",
            config.constants.base_url,
            self.get_str("Path").unwrap()
        ))
        .expect("lib error, could not find url for request type")
    }
}

trait DefaultEndpoint {
    fn default() -> Self;
}

impl DefaultEndpoint for Endpoint {
    fn default() -> Self {
        Endpoint::Authentication
    }
}

static CONFIG: OnceLock<EndpointsConfig> = OnceLock::new();

fn get() -> &'static EndpointsConfig {
    CONFIG.get_or_init(|| toml::from_str(RAW).expect("endpoint config error"))
}
