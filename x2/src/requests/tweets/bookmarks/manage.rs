use super::prelude::*;

use crate::{requests::ClientAgnosticBuilder, responses::tweets::bookmarks::manage::Response};

type UserId<'a> = &'a str;
type TweetId<'a> = &'a str;

#[derive(Serialize)]
struct BookmarkPostBody<'a> {
    tweet_id: TweetId<'a>,
}

#[derive(Debug, Built, Authorized)]
pub struct Request {
    builder: Option<RequestBuilder>,
}

pub enum Action<'a> {
    Bookmark(UserId<'a>, TweetId<'a>),
    Unbookmark(UserId<'a>, TweetId<'a>),
}

impl<'a> Action<'_> {
    pub fn effect(self, client: reqwest_oauth1::Client<DefaultSigner>) -> RequestBuilder {
        ClientAgnosticBuilder::Oauth1(match self {
            Action::Bookmark(user_id, tweet_id) => client
                .post(super::Endpoint::ManageAddBookmark.url(Some(&[user_id])))
                .json(&BookmarkPostBody { tweet_id }),
            Action::Unbookmark(source, target) => {
                client.delete(super::Endpoint::ManageRemoveBookmark.url(Some(&[source, target])))
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

    pub fn bookmark(auth: &Context, user_id: &str, tweet_id: &str) -> Self {
        Self::new(auth, Action::Bookmark(user_id, tweet_id))
    }

    pub fn unbookmark(auth: &Context, user_id: &str, tweet_id: &str) -> Self {
        Self::new(auth, Action::Unbookmark(user_id, tweet_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_util::oauth1_credentials;

    #[test]
    fn bookmarks_manage<'a>() {
        let context = oauth1_credentials();

        let response = Request::bookmark(&context, "1444148135954108418", "13000192").request();

        let response2 = Request::unbookmark(&context, "1444148135954108418", "13000192").request();

        println!("{:?} {:?}", response, response2);

        assert!(response.is_ok());
    }
}
