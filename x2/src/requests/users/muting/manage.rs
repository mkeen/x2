use super::prelude::*;

use crate::responses::users::muting::Response;

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

pub struct Request {
    builder: Option<RequestBuilder>,
}

pub enum Action<'a> {
    Mute(SourceUserId<'a>, TargetUserId<'a>),
    Unmute(SourceUserId<'a>, TargetUserId<'a>),
}

impl<'a> Action<'_> {
    pub fn effect(self, client: &'static reqwest::blocking::Client) -> RequestBuilder {
        match self {
            Action::Mute(source, target) => client
                .post(super::Endpoint::Unmute.url(Some(&[source])))
                .json(&MutePostBody {
                    target_user_id: target,
                }),
            Action::Unmute(source, target) => {
                client.delete(super::Endpoint::Unmute.url(Some(&[source, target])))
            }
        }
    }
}

impl Authorized<Response> for Request {}

impl Request {
    pub fn new(auth: &Context, action: Action) -> Self {
        Self {
            builder: Self::builder_with_auth(auth, action.effect(client())),
        }
    }

    pub fn mute(auth: &Context, source_user_id: &str, target_user_id: &str) -> Self {
        Self::new(auth, Action::Mute(source_user_id, target_user_id))
    }

    pub fn unmute(auth: &Context, source_user_id: &str, target_user_id: &str) -> Self {
        Self::new(auth, Action::Unmute(source_user_id, target_user_id))
    }
}

impl super::Request<Response> for Request {
    fn builder(&mut self) -> Option<RequestBuilder> {
        self.builder.take()
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::auth, requests::Request as RequestTrait};

    use super::Request;

    #[test]
    fn integration_users_muting_manage_with_defaults() {
        let id = "c2HAMlWTX2m3cVgNgA0oqLRqH";
        let secret = "bwWKCB8KHHRnMDAKUa4cmZdp80FZxNsCLo2G1axDRHjb7nkOc2";

        let context = auth::Context::Caller(auth::Method::AppOnly { id, secret });

        // not testing authentication here, so will just unwrap and assume all is well
        let authorization = context.authorize().unwrap();

        let response = Request::unmute(&authorization, "123", "123").request();

        println!("{:?}", response);

        assert!(response.is_ok());

        let response = response.unwrap();

        assert!(!response.data.muting)
    }
}
