use super::prelude::{Deserialize, EnumCount, IntoStaticStr};

#[derive(Deserialize, IntoStaticStr, EnumCount, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "description")]
    Description,
}
