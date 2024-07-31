use super::prelude::Deserialize;

use geojson::GeoJson;

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
