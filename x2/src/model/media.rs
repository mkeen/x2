use super::prelude::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metrics {
    pub playback_0_count: Option<u64>,
    pub playback_100_count: Option<u64>,
    pub playback_25_count: Option<u64>,
    pub playback_50_count: Option<u64>,
    pub playback_75_count: Option<u64>,
    pub view_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    pub bit_rate: Option<u64>,
    pub content_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Media {
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub media_key: Option<String>,
    pub url: Option<String>,
    pub duration_ms: Option<u64>,
    pub height: Option<usize>,
    pub width: Option<usize>,
    pub non_public_metrics: Option<Metrics>,
    pub organic_metrics: Option<Metrics>,
    pub preview_image_url: Option<String>,
    pub promoted_metrics: Option<Metrics>,
    pub public_metrics: Option<Metrics>,
    pub alt_text: Option<String>,
    pub variants: Option<Vec<Variant>>,
}
