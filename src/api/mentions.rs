use serde_qs::to_string as convert_query_to_string;
use crate::api::client::TweetyClient;
use serde::{Deserialize, Serialize};
use crate::api::error::TweetyError;
use reqwest::Method;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryParams {
    pub end_time: Option<String>,               // ISO 8601 date string
    pub expansions: Option<Vec<ExpansionType>>, // List of enum values
    pub max_results: Option<u32>,               // Integer for max results, min 5, max 100
    pub media_fields: Option<Vec<MediaField>>,  // List of enum values
    pub pagination_token: Option<String>,       // String for pagination token
    pub place_fields: Option<Vec<PlaceField>>,  // List of enum values
    pub poll_fields: Option<Vec<PollField>>,    // List of enum values
    pub since_id: Option<String>,               // String for since_id (Tweet ID)
    pub start_time: Option<String>,             // ISO 8601 date string
    pub tweet_fields: Option<Vec<TweetField>>,  // List of enum values
    pub until_id: Option<String>,               // String for until_id (Tweet ID)
    pub user_fields: Option<Vec<UserField>>,    // List of enum values
}

// Enum for the `expansions` parameter
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpansionType {
    AttachmentsPollIds,
    AttachmentsMediaKeys,
    AuthorId,
    EditHistoryTweetIds,
    EntitiesMentionsUsername,
    GeoPlaceId,
    InReplyToUserId,
    ReferencedTweetsId,
    ReferencedTweetsIdAuthorId,
}

// Enum for the `media_fields` parameter
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaField {
    DurationMs,
    Height,
    MediaKey,
    PreviewImageUrl,
    Type,
    Url,
    Width,
    PublicMetrics,
    NonPublicMetrics,
    OrganicMetrics,
    PromotedMetrics,
    AltText,
    Variants,
}

// Enum for the `place_fields` parameter
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceField {
    ContainedWithin,
    Country,
    CountryCode,
    FullName,
    Geo,
    Id,
    Name,
    PlaceType,
}

// Enum for the `poll_fields` parameter
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PollField {
    DurationMinutes,
    EndDatetime,
    Id,
    Options,
    VotingStatus,
}

// Enum for the `tweet_fields` parameter
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TweetField {
    Attachments,
    AuthorId,
    ContextAnnotations,
    ConversationId,
    CreatedAt,
    EditControls,
    Entities,
    Geo,
    Id,
    InReplyToUserId,
    Lang,
    NonPublicMetrics,
    PublicMetrics,
    OrganicMetrics,
    PromotedMetrics,
    PossiblySensitive,
    ReferencedTweets,
    ReplySettings,
    Source,
    Text,
    Withheld,
}

// Enum for the `user_fields` parameter
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserField {
    CreatedAt,
    Description,
    Entities,
    Id,
    Location,
    MostRecentTweetId,
    Name,
    PinnedTweetId,
    ProfileImageUrl,
    Protected,
    PublicMetrics,
    Url,
    Username,
    Verified,
    VerifiedType,
    Withheld,
}

// Root Response Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct MentionsResponse {
    pub data: Vec<TweetData>,       // Required field for tweet data
    pub includes: Option<Includes>, // Optional field for included related data
    pub meta: MetaData,             // Required field for metadata
}

// Struct for each Tweet in "data"
#[derive(Debug, Serialize, Deserialize)]
pub struct TweetData {
    pub author_id: String,                   // Required field for author ID
    pub text: String,                        // Required field for tweet text
    pub lang: String,                        // Required field for language
    pub conversation_id: String,             // Required field for conversation ID
    pub edit_history_tweet_ids: Vec<String>, // Required field for edit history tweet IDs
    pub id: String,                          // Required field for tweet ID
}

// Struct for "includes"
#[derive(Debug, Serialize, Deserialize)]
pub struct Includes {
    pub users: Vec<UserData>, // Optional field for included users data
}

// Struct for each User in "includes.users"
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub name: String,                   // Required field for user name
    pub id: String,                     // Required field for user ID
    pub entities: Option<UserEntities>, // Optional field for user entities
    pub username: String,               // Required field for user username
    pub created_at: Option<String>,     // Optional field for user creation time (ISO 8601)
}

// Struct for "entities" in UserData
#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntities {
    pub url: Option<UrlEntity>, // Optional field for URL entities
    pub description: Option<DescriptionEntity>, // Optional field for description entities
}

// Struct for "url" in UserEntities
#[derive(Debug, Serialize, Deserialize)]
pub struct UrlEntity {
    pub urls: Vec<UrlDetail>, // Required field for URLs
}

// Struct for each URL in "url.urls"
#[derive(Debug, Serialize, Deserialize)]
pub struct UrlDetail {
    pub start: u32,           // Required field for start position
    pub end: u32,             // Required field for end position
    pub url: String,          // Required field for URL
    pub expanded_url: String, // Required field for expanded URL
    pub display_url: String,  // Required field for display URL
}

// Struct for "description" in UserEntities
#[derive(Debug, Serialize, Deserialize)]
pub struct DescriptionEntity {
    pub urls: Option<Vec<UrlDetail>>, // Optional field for URLs in description
    pub hashtags: Option<Vec<Hashtag>>, // Optional field for hashtags in description
    pub mentions: Option<Vec<Mention>>, // Optional field for mentions in description
    pub cashtags: Option<Vec<Cashtag>>, // Optional field for cashtags in description
}

// Struct for each hashtag in DescriptionEntity
#[derive(Debug, Serialize, Deserialize)]
pub struct Hashtag {
    pub start: u32,  // Required field for start position
    pub end: u32,    // Required field for end position
    pub tag: String, // Required field for hashtag
}

// Struct for each mention in DescriptionEntity
#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    pub start: u32,       // Required field for start position
    pub end: u32,         // Required field for end position
    pub username: String, // Required field for username
}

// Struct for each cashtag in DescriptionEntity
#[derive(Debug, Serialize, Deserialize)]
pub struct Cashtag {
    pub start: u32,  // Required field for start position
    pub end: u32,    // Required field for end position
    pub tag: String, // Required field for cashtag
}

// Struct for "meta" in the response
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub oldest_id: String,          // Required field for oldest tweet ID
    pub newest_id: String,          // Required field for newest tweet ID
    pub result_count: u32,          // Required field for result count
    pub next_token: Option<String>, // Optional field for pagination token
}

impl TweetyClient {
    /// Authentication methods
    /// supported by this endpoint
    /// OAuth 1.0a is also available for this endpoint.
    /// OAuth 2.0 Authorization Code with PKCE
    /// OAuth 2.0 App-only
    /// [Docs](https://developer.x.com/en/docs/x-api/tweets/timelines/api-reference/get-users-id-mentions)
    pub async fn get_users_mentions(
        &self,
        user_id: &str,
        query_params: Option<QueryParams>,
    ) -> Result<MentionsResponse, TweetyError> {
        let mut base_url = format!("https://api.x.com/2/users/{}/mentions", user_id);
        if let Some(queries) = query_params {
            let query_params = convert_query_to_string(&queries).unwrap();
            base_url = format!("{}?{}", base_url, query_params);
        }
        match self.send_request::<()>(&base_url, Method::GET, None).await {
            Ok(value) => match serde_json::from_value::<MentionsResponse>(value) {
                Ok(data) => Ok(data),
                Err(err) => Err(TweetyError::JsonParseError(err.to_string())),
            },
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
}
