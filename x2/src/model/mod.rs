pub(crate) mod prelude {
    pub use super::super::prelude::*;
}

pub mod auth;
pub mod entities;
pub mod error;
pub mod media;
pub mod places;
pub mod polls;
pub mod rate_limit;
pub mod spaces;
pub mod topics;
pub mod tweets;
pub mod users;
pub mod withheld;

// pub trait Inclusive<'a, T> {
//     fn includes(&'a mut self, include: &'a T) -> &'a impl Inclusive<T>;
// }
