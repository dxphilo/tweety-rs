use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ApiResponse {
    pub data: Vec<UserResponse>,
    pub includes: Option<Includes>,
}

#[derive(Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
    pub id: String,
}

#[derive(Deserialize)]
pub struct Includes {
    pub tweets: Vec<Tweet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub username: String,
    pub created_at: Option<String>, // ISO 8601 date format
    pub most_recent_tweet_id: Option<String>,
    pub protected: Option<bool>,
    pub withheld: Option<Withheld>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub verified: Option<bool>,
    pub verified_type: Option<VerifiedType>, // Enum could be used here
    pub entities: Option<Entities>,
    pub profile_image_url: Option<String>,
    pub public_metrics: Option<PublicMetrics>,
    pub pinned_tweet_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerifiedType {
    Blue,
    Business,
    Government,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Withheld {
    pub country_codes: Option<Vec<String>>,
    pub scope: Option<WithheldScope>, // Enum could be used here
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WithheldScope {
    Tweet,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entities {
    pub url: Option<UrlEntities>,
    pub description: Option<DescriptionEntities>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlEntities {
    pub urls: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub url: Option<String>,
    pub expanded_url: Option<String>,
    pub display_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DescriptionEntities {
    pub urls: Option<Vec<Url>>,
    pub hashtags: Option<Vec<Hashtag>>,
    pub mentions: Option<Vec<Mention>>,
    pub cashtags: Option<Vec<Cashtag>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hashtag {
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub hashtag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cashtag {
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub cashtag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicMetrics {
    pub followers_count: Option<u32>,
    pub following_count: Option<u32>,
    pub tweet_count: Option<u32>,
    pub listed_count: Option<u32>,
}
