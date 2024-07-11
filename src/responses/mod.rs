use strum::EnumDiscriminants;

use crate::model::auth::RequestCredential;

#[derive(EnumDiscriminants)]
#[strum_discriminants(name(ResponseId))]
pub enum Response<'a> {
    Bearer(RequestCredential<'a>),
}
