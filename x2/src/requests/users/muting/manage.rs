use super::prelude::*;

use crate::{requests::ClientAgnosticBuilder, responses::users::muting::manage::Response};

type UserId<'a> = &'a str;
type SourceUserId<'a> = UserId<'a>;
type TargetUserId<'a> = UserId<'a>;

fn validate_id(id: UserId) -> Result<(), LibError> {
    id.parse::<u64>()
        .map_err(|e| LibError::InvalidSnowflake(e))
        .map(|_| ())
}

#[derive(Serialize)]
struct MutePostBody<'a> {
    target_user_id: TargetUserId<'a>,
}

#[derive(Debug, Built, Authorized)]
pub struct Request {
    builder: Option<RequestBuilder>,
}

pub enum Action<'a> {
    Mute(SourceUserId<'a>, TargetUserId<'a>),
    Unmute(SourceUserId<'a>, TargetUserId<'a>),
}

impl<'a> Action<'_> {
    pub fn effect(self, client: reqwest_oauth1::Client<DefaultSigner>) -> RequestBuilder {
        ClientAgnosticBuilder::Oauth1(match self {
            Action::Mute(source, target) => client
                .post(super::Endpoint::Unmute.url(Some(&[source])))
                .json(&MutePostBody {
                    target_user_id: target,
                }),
            Action::Unmute(source, target) => {
                client.delete(super::Endpoint::Unmute.url(Some(&[source, target])))
            }
        })
    }
}

impl Request {
    pub fn new(auth: &Context, action: Action) -> Self {
        Self {
            builder: Some(action.effect(Self::authorize_oauth1(auth))),
        }
    }

    pub fn mute(auth: &Context, source_user_id: &str, target_user_id: &str) -> Self {
        Self::new(auth, Action::Mute(source_user_id, target_user_id))
    }

    pub fn unmute(auth: &Context, source_user_id: &str, target_user_id: &str) -> Self {
        Self::new(auth, Action::Unmute(source_user_id, target_user_id))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::auth::{Method, RequestCredential};

    use super::*;

    #[test]
    fn muting_manage<'a>() {
        let consumer_id = "c2HAMlWTX2m3cVgNgA0oqLRqH".to_string();
        let consumer_secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2".to_string();

        let oauth2_client_id = "TV9xZXRVVVN0STIwSkcwck9WS2w6MTpjaQ".to_string();
        let oauth2_client_secret = "gZHqK9YQZyrH7x7P9Yg5kxdE3j8_yDQopjBxXIptw-4b2TIM4_".to_string();

        let user_id = "1444148135954108418-TSUe6cI1lpIddYScxSKIlmbfq71kyL".to_string();
        let user_secret = "vupepUIBVJl08dhMdlHuNTyRWaWUVPenrPpSl1E4EqWb6".to_string();

        let context = Context::Request(RequestCredential::OAuth10AConsumer {
            consumer_id,
            consumer_secret,
            user_id,
            user_secret,
        });

        let response = Request::unmute(&context, "1444148135954108418", "13000192").request();

        let response2 = Request::mute(&context, "1444148135954108418", "13000192").request();

        println!("{:?} {:?}", response, response2);

        assert!(response.is_ok());
    }
}
