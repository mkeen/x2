use strum::{Display, EnumString};

use crate::model::{
    auth::RequestCredential,
    error::XError,
    spaces::{Expansion as SpaceExpansion, Field as SpaceField, State as SpaceState},
    user::Field as UserField,
};

#[derive(Clone, EnumString, Display)]
pub enum TopicField {
    #[strum(to_string = "id")]
    Id,
    #[strum(to_string = "name")]
    Name,
    #[strum(to_string = "all")]
    Description,
}

pub struct Request<'a> {
    client: reqwest::blocking::Client,
    credential: RequestCredential,
    query: &'a str,
    expansions: Vec<SpaceExpansion>,
    space_fields: Vec<SpaceField>,
    user_fields: Vec<UserField>,
    topic_fields: Vec<TopicField>,
    state: SpaceState,
    // usage: UsageScope,
}

impl<'a> Request<'a> {
    pub fn new(
        credential: RequestCredential,
        query: &'a str,
        expansions: Option<Vec<SpaceExpansion>>,
        space_fields: Option<Vec<SpaceField>>,
        user_fields: Option<Vec<UserField>>,
        topic_fields: Option<Vec<TopicField>>,
        state: Option<SpaceState>,
    ) -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .user_agent(super::APP_USER_AGENT)
                .referer(false)
                .https_only(true)
                .gzip(true)
                .build()
                .unwrap(),
            credential,
            expansions: expansions.unwrap_or(vec![]),
            space_fields: space_fields.unwrap_or(vec![]),
            user_fields: user_fields.unwrap_or(vec![]),
            topic_fields: topic_fields.unwrap_or(vec![]),
            state: state.unwrap_or(SpaceState::All),
            query,
        }
    }
}

impl<'a> super::Request<'a> for Request<'a> {
    #[allow(refining_impl_trait)]
    fn request(
        &self,
    ) -> Result<crate::responses::spaces_search::Response, crate::model::error::XError> {
        match &(self.credential) {
            RequestCredential::Bearer(bearer) => {
                let mut params: Vec<(String, String)> = vec![("query".into(), self.query.into())];

                if self.expansions.len() > 0 {
                    let expansions: Vec<String> = (&self.expansions)
                        .into_iter()
                        .map(|e| e.to_string())
                        .collect();

                    params.push(("expansions".into(), expansions.join(",")));
                }

                if self.space_fields.len() > 0 {
                    let space_fields: Vec<String> = (&self.space_fields)
                        .into_iter()
                        .map(|f| f.to_string())
                        .collect();

                    params.push(("space.fields".into(), space_fields.join(",")));
                }

                if self.user_fields.len() > 0 {
                    let user_fields: Vec<String> = (&self.user_fields)
                        .into_iter()
                        .map(|f| f.to_string())
                        .collect();

                    params.push(("user.fields".into(), user_fields.join(",")));
                }

                if self.topic_fields.len() > 0 {
                    let topic_fields: Vec<String> = (&self.topic_fields)
                        .into_iter()
                        .map(|f| f.to_string())
                        .collect();

                    params.push(("topic.fields".into(), topic_fields.join(",")));
                }

                let r = self
                    .client
                    .get(crate::config::Endpoint::SpacesSearch.url())
                    .query(&params)
                    .bearer_auth(bearer);

                println!("{:?}", r);

                let res = r
                    .send()
                    .map_err(|e| XError::Generic(e.status().unwrap_or_default(), e.to_string()))?;

                match res.status().is_success() {
                    true => crate::responses::spaces_search::Response::try_from_bytes(
                        &res.bytes().map_err(|e| XError::Reqwest(e))?,
                    ),
                    false => Err(XError::Generic(
                        res.status(),
                        res.text().unwrap_or("Unknown".into()),
                    )),
                }
            }
        }
    }
}
