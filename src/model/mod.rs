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

pub trait Inclusive<'a, T> {
    fn includes(&'a mut self, include: &'a T) -> &'a impl Inclusive<T>;
}

// pub enum ParamValue {
//     Single(String),
//     Multi(Vec<String>),
// }

// impl ParamValue {
//     fn csv(&self) -> &str {
//         match self {
//             Self::Single(v) => v,
//             Self::Multi(v) => &v.join(","),
//         }
//     }
// }
