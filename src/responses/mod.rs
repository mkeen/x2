use crate::model::auth::RequestCredential;

pub enum Response {
    AuthToken(RequestCredential),
}
