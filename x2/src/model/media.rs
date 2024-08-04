use super::prelude::{Deserialize, EnumCount, IntoStaticStr};

#[derive(Debug, Deserialize)]
pub struct Metrics {
    pub playback_0_count: Option<u64>,
    pub playback_100_count: Option<u64>,
    pub playback_25_count: Option<u64>,
    pub playback_50_count: Option<u64>,
    pub playback_75_count: Option<u64>,
    pub view_count: Option<u64>,
}

#[derive(Deserialize, IntoStaticStr, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "duration_ms")]
    DurationMs,
    #[strum(serialize = "height")]
    Height,
    #[strum(serialize = "media_key")]
    MediaKey,
    #[strum(serialize = "preview_image_url")]
    PreviewImageUrl,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "url")]
    Url,
    #[strum(serialize = "width")]
    Width,
    #[strum(serialize = "public_metrics")]
    PublicMetrics,
    #[strum(serialize = "non_public_metrics")]
    NonPublicMetrics,
    #[strum(serialize = "organic_metrics")]
    OrganicMetrics,
    #[strum(serialize = "promoted_metrics")]
    PromotedMetrics,
    #[strum(serialize = "alt_text")]
    AltText,
    #[strum(serialize = "variants")]
    Variants,
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
