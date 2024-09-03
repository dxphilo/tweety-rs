use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_qs::to_string as query_string;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryParams {
    // Required parameter
    pub query: String,

    // Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>, // ISO 8601 date string

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansions: Option<Vec<Expansion>>, // List of expansions

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u8>, // Integer between 10 and 100

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_fields: Option<Vec<MediaField>>, // List of media fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>, // String token for pagination

    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_fields: Option<Vec<PlaceField>>, // List of place fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_fields: Option<Vec<PollField>>, // List of poll fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>, // String for tweet ID

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>, // Enum for recency or relevancy

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>, // ISO 8601 date string

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweet_fields: Option<Vec<TweetField>>, // List of tweet fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>, // String for tweet ID

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<Vec<UserField>>, // List of user fields
}

// Enums for the various fields

#[derive(Debug, Serialize, Deserialize)]
pub enum Expansion {
    #[serde(rename = "attachments.poll_ids")]
    AttachmentsPollIds,
    #[serde(rename = "attachments.media_keys")]
    AttachmentsMediaKeys,
    #[serde(rename = "author_id")]
    AuthorId,
    #[serde(rename = "edit_history_tweet_ids")]
    EditHistoryTweetIds,
    #[serde(rename = "entities.mentions.username")]
    EntitiesMentionsUsername,
    #[serde(rename = "geo.place_id")]
    GeoPlaceId,
    #[serde(rename = "in_reply_to_user_id")]
    InReplyToUserId,
    #[serde(rename = "referenced_tweets.id")]
    ReferencedTweetsId,
    #[serde(rename = "referenced_tweets.id.author_id")]
    ReferencedTweetsIdAuthorId,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaField {
    #[serde(rename = "duration_ms")]
    DurationMs,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "media_key")]
    MediaKey,
    #[serde(rename = "preview_image_url")]
    PreviewImageUrl,
    #[serde(rename = "type")]
    Type,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "width")]
    Width,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "non_public_metrics")]
    NonPublicMetrics,
    #[serde(rename = "organic_metrics")]
    OrganicMetrics,
    #[serde(rename = "promoted_metrics")]
    PromotedMetrics,
    #[serde(rename = "alt_text")]
    AltText,
    #[serde(rename = "variants")]
    Variants,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlaceField {
    #[serde(rename = "contained_within")]
    ContainedWithin,
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "country_code")]
    CountryCode,
    #[serde(rename = "full_name")]
    FullName,
    #[serde(rename = "geo")]
    Geo,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "place_type")]
    PlaceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PollField {
    #[serde(rename = "duration_minutes")]
    DurationMinutes,
    #[serde(rename = "end_datetime")]
    EndDateTime,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "options")]
    Options,
    #[serde(rename = "voting_status")]
    VotingStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "recency")]
    Recency,
    #[serde(rename = "relevancy")]
    Relevancy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TweetField {
    #[serde(rename = "attachments")]
    Attachments,
    #[serde(rename = "author_id")]
    AuthorId,
    #[serde(rename = "context_annotations")]
    ContextAnnotations,
    #[serde(rename = "conversation_id")]
    ConversationId,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "edit_controls")]
    EditControls,
    #[serde(rename = "entities")]
    Entities,
    #[serde(rename = "geo")]
    Geo,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "in_reply_to_user_id")]
    InReplyToUserId,
    #[serde(rename = "lang")]
    Lang,
    #[serde(rename = "non_public_metrics")]
    NonPublicMetrics,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "organic_metrics")]
    OrganicMetrics,
    #[serde(rename = "promoted_metrics")]
    PromotedMetrics,
    #[serde(rename = "possibly_sensitive")]
    PossiblySensitive,
    #[serde(rename = "referenced_tweets")]
    ReferencedTweets,
    #[serde(rename = "reply_settings")]
    ReplySettings,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "withheld")]
    Withheld,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserField {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "entities")]
    Entities,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "most_recent_tweet_id")]
    MostRecentTweetId,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "pinned_tweet_id")]
    PinnedTweetId,
    #[serde(rename = "profile_image_url")]
    ProfileImageUrl,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "public_metrics")]
    PublicMetrics,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "username")]
    Username,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "verified_type")]
    VerifiedType,
    #[serde(rename = "withheld")]
    Withheld,
}

impl QueryParams {
    pub fn to_query_string(&self) -> String {
        query_string(self).unwrap()
    }
}

// RESPONSE STRUCT

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentSearchResponse {
    pub data: Vec<TweetData>,
    pub includes: Option<Includes>,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetData {
    pub text: String,
    pub author_id: String,
    pub id: String,
    pub edit_history_tweet_ids: Vec<String>,
    pub lang: String,
    pub conversation_id: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Includes {
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub entities: Option<UserEntities>,
    pub created_at: String,
    pub username: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntities {
    pub url: Option<UserUrl>,
    pub description: Option<UserDescription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUrl {
    pub urls: Vec<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub start: u32,
    pub end: u32,
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDescription {
    pub hashtags: Vec<Hashtag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hashtag {
    pub start: u32,
    pub end: u32,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub newest_id: String,
    pub oldest_id: String,
    pub result_count: u32,
}

impl TweetyClient {
    /// Search for Posts published in the last 7 days
    /// The recent search endpoint returns Tweets from the last seven days that match a search query.
    /// [Docs](https://developer.x.com/en/docs/x-api/tweets/search/api-reference/get-tweets-search-recent#tab1)
    pub async fn recent_search(
        &self,
        query: &str,
        query_params: Option<QueryParams>,
    ) -> Result<RecentSearchResponse, TweetyError> {
        let mut base_url = format!("https://api.x.com/2/tweets/search/recent?query={}", query);

        if let Some(value) = query_params {
            base_url = format!("{}&{}", base_url, value.to_query_string());
        }

        match self.send_request::<()>(&base_url, Method::GET, None).await {
            Ok(value) => {
                let json_data = serde_json::from_value::<RecentSearchResponse>(value);

                if let Ok(data) = json_data {
                    Ok(data)
                } else {
                    Err(TweetyError::JsonParseError("Invalid JSON".to_string()))
                }
            }
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
    /// Only available to those with Pro and Enterprise access
    /// Search the full archive of Posts
    /// Authentication methods
    /// supported by this endpoint OAuth 2.0 App-only
    /// [Docs](https://developer.x.com/en/docs/x-api/tweets/search/api-reference/get-tweets-search-all)
    pub async fn full_archive_search(
        &self,
        query: &str,
        query_params: Option<QueryParams>,
    ) -> Result<Value, TweetyError> {
        let mut base_url = format!("https://api.x.com/2/tweets/search/all?query={}", query);

        if let Some(queries) = query_params {
            base_url = format!("{}&{}", base_url, queries.to_query_string());
        }

        match self.send_request::<()>(&base_url, Method::GET, None).await {
            Ok(value) => Ok(value),
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
}
