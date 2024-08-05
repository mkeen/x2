use super::prelude::*;

use model::auth::{Context, RequestCredential};

use serde::{de::Visitor, Deserializer};
use std::fmt;

#[derive(Deserialize, Debug, PartialEq, Eq, EnumIs)]
#[serde(tag = "token_type", rename_all = "snake_case")]
pub enum Response {
    #[serde(deserialize_with = "deserialize_response")]
    Bearer(String),
}

impl Into<Context> for Response {
    fn into(self) -> Context {
        match self {
            Response::Bearer(bearer) => Context::Request(RequestCredential::Bearer(bearer)),
        }
    }
}

fn deserialize_response<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringVisitor;

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representing the access token")
        }

        fn visit_map<V>(self, mut map: V) -> Result<String, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            while let Some(key) = map.next_key()? {
                match key {
                    "access_token" => return map.next_value(),
                    _ => {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
            }
            Err(serde::de::Error::missing_field("access_token"))
        }
    }

    deserializer.deserialize_map(StringVisitor)
}

impl super::Response for Response {
    type Request = requests::auth::Request;
}
