use super::prelude::*;

use crate::{requests::ClientAgnosticBuilder, responses::users::muting::manage::Response};

type UserId<'a> = &'a str;
type SourceUserId<'a> = UserId<'a>;
type TargetUserId<'a> = UserId<'a>;

#[derive(Serialize)]
struct MutePostBody<'a> {
    target_user_id: TargetUserId<'a>,
}

#[derive(Debug, Built, Authorized)]
pub struct Request<'a> {
    builder: Option<RequestBuilder<'a>>,
}

pub enum Action<'a> {
    Mute(SourceUserId<'a>, TargetUserId<'a>),
    Unmute(SourceUserId<'a>, TargetUserId<'a>),
}

impl<'a> Action<'_> {
    pub fn effect(self, client: reqwest_oauth1::Client<DefaultSigner>) -> RequestBuilder {
        ClientAgnosticBuilder::Oauth1(match self {
            Action::Mute(source, target) => client
                .post(super::Endpoint::Mute.url(Some(&[source])))
                .json(&MutePostBody {
                    target_user_id: target,
                }),
            Action::Unmute(source, target) => {
                client.delete(super::Endpoint::Unmute.url(Some(&[source, target])))
            }
        })
    }
}

impl<'a> Request<'a> {
    pub fn new(auth: &'a Context, action: Action) -> Self {
        Self {
            builder: Some(action.effect(Self::authorize_oauth1(auth))),
        }
    }

    pub fn mute(auth: &'a Context, source_user_id: &str, target_user_id: &str) -> Self {
        Self::new(auth, Action::Mute(source_user_id, target_user_id))
    }

    pub fn unmute(auth: &'a Context, source_user_id: &str, target_user_id: &str) -> Self {
        Self::new(auth, Action::Unmute(source_user_id, target_user_id))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::oauth1_credentials;

    use super::*;

    #[test]
    fn muting_manage() {
        let context = oauth1_credentials();

        let mute = Request::mute(&context, "1444148135954108418", "20786413").request();
        let unmute = Request::unmute(&context, "1444148135954108418", "20786413").request();

        assert!(mute.is_ok() && unmute.is_ok());
    }
}
