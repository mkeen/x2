use reqwest::Url;
use serde::Deserialize;
use std::sync::OnceLock;
use strum::{EnumDiscriminants, EnumIter};
use toml;

const RAW: &str = include_str!("./../api.toml");

#[derive(Debug, Deserialize)]
pub struct EndpointsConfig {
    constants: Constants,
    endpoints: Vec<Endpoint>,
}

impl EndpointsConfig {
    pub fn find_endpoint(&self, search: EndpointId) -> Option<&Endpoint> {
        for endpoint in &self.endpoints {
            let endpoint_id: EndpointId = endpoint.into();
            if search == endpoint_id {
                return Some(endpoint);
            }
        }

        return None;
    }
}

#[derive(Debug, Deserialize)]
pub struct Constants {
    version: String,
    base_url: String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct EndpointData {
    path: String,
}

#[derive(Debug, Deserialize, PartialEq, EnumDiscriminants, EnumIter)]
#[strum_discriminants(name(EndpointId))]
#[serde(tag = "request")]
pub enum Endpoint {
    Authentication(EndpointData),
    RateLimit(EndpointData),
}

trait DefaultEndpoint {
    fn default() -> Self;
}

impl Endpoint {
    pub fn url(&self, constants: &Constants) -> Url {
        Url::parse(&format!(
            "https://{}/{}/{}",
            constants.base_url,
            constants.version,
            match self {
                Self::Authentication(d) => d.path.clone(),
                Self::RateLimit(d) => d.path.clone(),
            }
        ))
        .unwrap()
    }
}

impl DefaultEndpoint for Endpoint {
    fn default() -> Self {
        Endpoint::Authentication(EndpointData::default())
    }
}

static CONFIG: OnceLock<EndpointsConfig> = OnceLock::new();

fn get() -> &'static EndpointsConfig {
    CONFIG.get_or_init(|| toml::from_str(RAW).expect("endpoint config error"))
}

pub fn endpoint_url(endpoint_type: EndpointId) -> Option<Url> {
    let config = get();
    config
        .find_endpoint(endpoint_type)
        .map(|e| e.url(&config.constants))
}
