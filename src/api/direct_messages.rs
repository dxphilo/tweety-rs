use super::error::TweetyError;
use crate::TweetyClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DMEventField {
    Id,
    Text,
    EventType,
    CreatedAt,
    DmConversationId,
    SenderId,
    ParticipantIds,
    ReferencedTweets,
    Attachments,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    MessageCreate,
    ParticipantsJoin,
    ParticipantsLeave,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expansion {
    AttachmentsMediaKeys,
    ReferencedTweetsId,
    SenderId,
    ParticipantIds,
}

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
    AltText,
    Variants,
}

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
    PublicMetrics,
    PossiblySensitive,
    ReferencedTweets,
    ReplySettings,
    Source,
    Text,
    Withheld,
}

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
    Withheld,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    #[serde(rename = "dm_event.fields")]
    pub dm_event_fields: Option<Vec<DMEventField>>,

    #[serde(rename = "event_types")]
    pub event_types: Option<Vec<EventType>>,

    #[serde(rename = "expansions")]
    pub expansions: Option<Vec<Expansion>>,

    /// The maximum number of results to be returned in a page. Must be between 1 and 100. The default is 100.
    #[serde(rename = "max_results")]
    pub max_results: Option<u32>,

    #[serde(rename = "media.fields")]
    pub media_fields: Option<Vec<MediaField>>,

    /// Contains either the next_token or previous_token value.
    #[serde(rename = "pagination_token")]
    pub pagination_token: Option<String>,

    #[serde(rename = "tweet.fields")]
    pub tweet_fields: Option<Vec<TweetField>>,

    #[serde(rename = "user.fields")]
    pub user_fields: Option<Vec<UserField>>,
}

/// [Docs](https://developer.x.com/en/docs/x-api/direct-messages/lookup/api-reference/get-dm_events)
///
impl QueryParams {
    pub fn to_query_string(&self) -> String {
        let mut params = vec![];

        if let Some(ref fields) = self.dm_event_fields {
            let fields_str = fields
                .iter()
                .map(|f| serde_json::to_string(f).unwrap())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("dm_event.fields={}", fields_str));
        }

        if let Some(ref types) = self.event_types {
            let types_str = types
                .iter()
                .map(|t| serde_json::to_string(t).unwrap())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("event_types={}", types_str));
        }

        if let Some(ref exps) = self.expansions {
            let exps_str = exps
                .iter()
                .map(|e| serde_json::to_string(e).unwrap())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("expansions={}", exps_str));
        }

        if let Some(max_results) = self.max_results {
            params.push(format!("max_results={}", max_results));
        }

        if let Some(ref media_fields) = self.media_fields {
            let media_str = media_fields
                .iter()
                .map(|m| serde_json::to_string(m).unwrap())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("media.fields={}", media_str));
        }

        if let Some(ref token) = self.pagination_token {
            params.push(format!("pagination_token={}", token));
        }

        if let Some(ref tweet_fields) = self.tweet_fields {
            let tweet_str = tweet_fields
                .iter()
                .map(|t| serde_json::to_string(t).unwrap())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("tweet.fields={}", tweet_str));
        }

        if let Some(ref user_fields) = self.user_fields {
            let user_str = user_fields
                .iter()
                .map(|u| serde_json::to_string(u).unwrap())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("user.fields={}", user_str));
        }

        params.join("&")
    }
}

impl TweetyClient {
    /// Returns a list of Direct Messages for the authenticated user, both sent and received. Direct
    /// Message events are returned in reverse chronological order.
    /// Supports retrieving events from the previous 30 days.
    /// Authentication methods supported by this endpoint
    // OAuth 2.0 Authorization Code with PKCE
    pub async fn get_direct_messages(&self, params: QueryParams) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/dm_events?{}", params.to_query_string());
        println!("Url: {}", url);
        self.send_request::<()>(&url, Method::GET, None).await
    }
    /// GET /2/dm_conversations/with/:participant_id/dm_events
    //Returns a list of Direct Messages (DM) events within a 1-1 conversation
    // with the user specified in the participant_id path parameter.
    /// Messages are returned in reverse chronological order.
    pub async fn get_dm_events_with_participant(
        &self,
        participant_id: &str,
        params: QueryParams,
    ) -> Result<Value, TweetyError> {
        let url = format!(
            "https://api.x.com/2/dm_conversations/with/{}/dm_events?{}",
            participant_id,
            params.to_query_string()
        );

        self.send_request::<()>(&url, Method::GET, None).await
    }
    /// Returns a list of Direct Messages within a conversation specified in the dm_conversation_id path parameter.
    /// Messages are returned in reverse chronological order.
    pub async fn get_dm_events_in_conversation(
        &self,
        dm_conversation_id: &str,
        params: QueryParams,
    ) -> Result<Value, TweetyError> {
        let url = format!(
            "https://api.x.com/2/dm_conversations/{}/dm_events?{}",
            dm_conversation_id,
            params.to_query_string()
        );

        self.send_request::<()>(&url, Method::GET, None).await
    }
}
