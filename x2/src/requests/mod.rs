pub(crate) mod prelude {
    pub use super::super::_prelude::*;
    pub use crate::config::Endpoint;
    pub use crate::model::auth::{Authorized, Context};
    pub use crate::responses::Response;
}

use prelude::*;
use rayon::prelude::*;

use arrayvec::ArrayVec;
use std::{collections::HashSet, sync::OnceLock, usize};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
static BASE_CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

pub mod auth;
//pub mod limits;
pub mod spaces;
//pub mod usage_tweets;
pub mod users;

pub type RequestBuilder = reqwest::blocking::RequestBuilder;

pub trait Request<T: Response> {
    fn request(self) -> Result<T, XError>;
}

// pub fn csv_par<const U: usize, T>(lists: [&[T]; U]) {
//     rayon::scope(|scope| {
//         for l in lists {
//             // rayon::spawn(|s| {

//             // })
//         }
//     });
// }

pub fn csv<const U: usize, T>(list: &[T]) -> String
where
    T: Clone + Into<&'static str>,
{
    list.iter()
        .map(|t| t.clone().into()) // todo: ugh.. clone.. not much that can be done tho
        .collect::<ArrayVec<&str, U>>()
        .join(",")
}

pub fn client() -> &'static reqwest::blocking::Client {
    BASE_CLIENT.get_or_init(|| {
        reqwest::blocking::ClientBuilder::new()
            .http2_prior_knowledge() // todo: remove, but look into other optimizations
            .user_agent(APP_USER_AGENT)
            .referer(false)
            .https_only(true)
            .gzip(true)
            .http1_title_case_headers()
            .build()
            .expect("failed to initialize the http client")
    })
}

#[cfg(test)]
mod tests {
    use crate::model::users::Field as UserField;
    use strum::EnumCount;

    use super::csv;

    #[test]
    fn csv_length() {
        let test_input = [
            UserField::CreatedAt,
            UserField::Description,
            UserField::Entities,
            UserField::Id,
        ];

        let c = csv::<{ UserField::COUNT }, UserField>(&test_input);

        assert_eq!(c.split(",").collect::<Vec<&str>>().len(), test_input.len());
    }
}
