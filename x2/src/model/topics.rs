use serde::Deserialize;
use strum::{AsRefStr, EnumCount};

#[derive(Deserialize, AsRefStr, EnumCount)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "description")]
    Description,
}
