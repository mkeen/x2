use super::prelude::{Deserialize, EnumCount, IntoStaticStr};

use geojson::GeoJson;

#[derive(Deserialize, IntoStaticStr, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "contained_within")]
    ContainedWithin,
    #[strum(serialize = "country")]
    Country,
    #[strum(serialize = "country_code")]
    CountryCode,
    #[strum(serialize = "full_name")]
    FullName,
    #[strum(serialize = "geo")]
    Geo,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "place_type")]
    PlaceType,
}

#[derive(Debug, Deserialize)]
pub struct Place {
    pub full_name: Option<String>,
    pub id: Option<String>,
    pub contained_within: Option<Vec<String>>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub geo: Option<GeoJson>,
    pub name: Option<String>,
    pub place_type: Option<String>,
}
