use serde::Deserialize;
use strum::{Display, EnumString};

#[derive(EnumString, Display, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(to_string = "id")]
    Id,
    #[strum(to_string = "name")]
    Name,
    #[strum(to_string = "description")]
    Description,
}
