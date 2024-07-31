use super::prelude::{Deserialize, IntoStaticStr};

#[derive(Debug, IntoStaticStr, Deserialize)]
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
