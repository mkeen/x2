use serde::Deserialize;
use strum::EnumString;

#[derive(Debug, EnumString, Deserialize)]
pub enum Scope {
    #[strum(to_string = "tweet")]
    Tweet,
    #[strum(to_string = "user")]
    User,
}

#[derive(Debug, Deserialize)]
pub struct Withheld {
    country_codes: Option<Vec<String>>,
    scope: Option<Scope>,
    copyright: Option<bool>,
}
