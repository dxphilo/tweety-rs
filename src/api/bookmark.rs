use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expansions {
    AttachmentsPollIds,
    AttachmentsMediaKeys,
    AuthorId,
    EditHistoryTweetIds,
    EntitiesMentionsUsername(String), // Allow username for mentions
    GeoPlaceId,
    InReplyToUserId,
    ReferencedTweetsId,
    ReferencedTweetsIdAuthorId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaFields {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceFields {
    ContainedWithin,
    Country,
    CountryCode,
    FullName,
    Geo,
    Id,
    Name,
    PlaceType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PollFields {
    DurationMinutes,
    EndDatetime,
    Id,
    Options,
    VotingStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TweetFields {
    Attachments,
    AuthorId,
    ContextAnnotations,
    ConversationId,
    CreatedAt,
    Entities,
    EditControls,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserFields {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkParams {
    pub expansions: Option<Vec<Expansions>>,
    pub max_results: Option<u32>, // Use u32 for integer parameters
    pub media_fields: Option<Vec<MediaFields>>,
    pub pagination_token: Option<String>,
    pub place_fields: Option<Vec<PlaceFields>>,
    pub poll_fields: Option<Vec<PollFields>>,
    pub tweet_fields: Option<Vec<TweetFields>>,
    pub user_fields: Option<Vec<UserFields>>,
}

/// # REFERENCE LINK
///
/// ## MANAGE BOOKMARKS
///
/// <https://developer.x.com/en/docs/x-api/tweets/bookmarks/api-reference>
impl TweetyClient {
    /// Bookmarks lookup
    /// Lookup a user's Bookmarks
    pub async fn get_user_bookmark(self, user_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/{}/bookmarks", user_id);

        self.send_request::<()>(&url, Method::GET, None).await
    }
    /// Bookmark a Post
    pub async fn bookmark_post(self, post_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/{}/bookmarks", post_id);

        self.send_request::<()>(&url, Method::POST, None).await
    }
    /// Remove a Bookmark of a Post
    pub async fn delete_bookmark(
        self,
        user_id: &str,
        tweet_id: &str,
    ) -> Result<Value, TweetyError> {
        let url = format!(
            "https://api.x.com/2/users/{}/bookmarks/{}",
            user_id, tweet_id
        );

        self.send_request::<()>(&url, Method::DELETE, None).await
    }
}
