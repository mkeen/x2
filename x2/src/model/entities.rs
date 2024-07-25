use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Annotation {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub probability: Option<u64>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub normalized_text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UrlEntityInfoContainer {
    pub urls: Option<Vec<UrlEntityInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct UrlEntityInfo {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub expanded_url: Option<String>,
    pub display_url: Option<String>,
    pub unwound_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DomainEntityInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub entity: Option<Entity>,
    pub expanded_url: Option<u8>,
    pub display_url: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct DomainEntityInfoContainer {
    pub domains: Option<Vec<DomainEntityInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct HashtagEntityInfo {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub hashtag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HashtagEntityInfoContainer {
    pub hashtags: Option<Vec<HashtagEntityInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct CashtagEntityInfo {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub cashtag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CashtagEntityInfoContainer {
    pub cashtags: Option<Vec<CashtagEntityInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct MentionEntityInfo {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub username: Option<String>,
    pub cashtags: Option<Vec<CashtagEntityInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct MentionEntityInfoContainer {
    pub mentions: Option<Vec<MentionEntityInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct Entities {
    url: Option<UrlEntityInfoContainer>,
    hashtags: Option<Vec<HashtagEntityInfoContainer>>,
    mentions: Option<Vec<MentionEntityInfoContainer>>,
    cashtags: Option<Vec<CashtagEntityInfoContainer>>,
    annotations: Option<Vec<Annotation>>,
}
