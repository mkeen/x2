use super::prelude::{Deserialize, XData};

#[derive(Debug, Deserialize, Eq, PartialEq, XData)]
pub struct Meta {
    count: Option<usize>,
    newest_id: Option<String>,
    oldest_id: Option<String>,
    next_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Data<D, I> {
    pub data: D,
    pub includes: Option<I>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleData<D> {
    pub data: D,
}
