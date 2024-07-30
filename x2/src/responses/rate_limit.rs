use std::collections::HashMap;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use serde_json::Value;

use crate::model::auth::RequestCredential;
use crate::model::error::XError;

pub use super::Response as ResponseTrait;

#[derive(Deserialize, Debug)]
pub struct Response {
    rate_limit_context: RateLimitContext,
    resources: HashMap<String, HashMap<String, HashMap<String, u128>>>,
}

#[derive(Debug)]
pub enum RateLimitContext {
    Application(String),
    AccessToken(RequestCredential),
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
                Err(de::Error::missing_field("application or access_token"))
            }
        }

        deserializer.deserialize_map(RateLimitContextVisitor)
    }
}

impl ResponseTrait for Response {
    type Response = Response;

    fn try_into_from_bytes(bytes: &[u8]) -> Result<Response, XError> {
        serde_json::from_slice::<Self>(bytes)
            .map_err(|e| XError::Deserialize(e))
            .map(|e| e)
    }
}
