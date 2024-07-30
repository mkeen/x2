use std::fmt;

use serde::{de::Visitor, Deserialize, Deserializer};
use strum::EnumIs;

use crate::model::error::XError;

pub use super::{Response as ResponseTrait, *};

#[derive(Deserialize, Debug, PartialEq, Eq, EnumIs)]
#[serde(tag = "token_type", rename_all = "snake_case")]
pub enum Response {
    #[serde(deserialize_with = "deserialize_response")]
    Bearer(String),
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

impl ResponseTrait for Response {
    type Response = Response;

    fn try_into_from_bytes(bytes: &[u8]) -> Result<Response, XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}
