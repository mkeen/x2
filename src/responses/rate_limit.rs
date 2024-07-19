use std::collections::HashMap;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use serde_json::Value;

use crate::model::auth::RequestCredential;
use crate::model::error::XError;

#[derive(Clone, Deserialize, Debug)]
pub struct Response {
    rate_limit_context: RateLimitContext,
    resources: HashMap<String, HashMap<String, u64>>,
}

#[derive(Clone, Debug)]
pub enum RateLimitContext {
    Application(String),
    AccessToken(RequestCredential),
}

impl Response {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, XError> {
        serde_json::from_slice::<Self>(bytes).map_err(|e| XError::Lib(e.to_string()))
    }
}

impl<'de> Deserialize<'de> for RateLimitContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RateLimitContextVisitor;

        impl<'de> Visitor<'de> for RateLimitContextVisitor {
            type Value = RateLimitContext;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a rate limit context object")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                while let Some(key) = map.next_key()? {
                    match key {
                        "application" => {
                            let value: String = map.next_value()?;
                            return Ok(RateLimitContext::Application(value));
                        }
                        "access_token" => {
                            let value: String = map.next_value()?;
                            return Ok(RateLimitContext::AccessToken(RequestCredential::Bearer(
                                value,
                            )));
                        }
                        _ => {
                            let _: Value = map.next_value()?;
                        }
                    }
                }
                Err(de::Error::missing_field("application"))
            }
        }

        deserializer.deserialize_map(RateLimitContextVisitor)
    }
}

impl<'a> super::Response<'a> for Response {}
