use serde::Deserialize;
use strum::AsRefStr;

#[derive(Debug, AsRefStr, Deserialize)]
pub enum Scope {
    #[strum(serialize = "tweet")]
    Tweet,
    #[strum(serialize = "user")]
    User,
}

#[derive(Debug, Deserialize)]
pub struct Withheld {
    country_codes: Option<Vec<String>>,
    scope: Option<Scope>,
    copyright: Option<bool>,
}
