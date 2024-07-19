use serde::{de::Visitor, Deserialize, Deserializer};
use std::fmt;

use crate::model::{error::XError};

#[derive(Clone, Deserialize)]
#[serde(tag = "token_type", rename_all = "lowercase")]
pub enum Response {
    #[serde(deserialize_with = "deserialize_response")]
    Bearer(String),
}

impl Response {
    pub fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, XError> {
        serde_json::from_slice::<Self>(&bytes)
            .map_err(|e| XError::ParseResponseFailed(e.to_string()))
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

impl<'a> super::Response<'a> for Response {}
