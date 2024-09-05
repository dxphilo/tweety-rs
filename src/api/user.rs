//! User Interaction API's
//!
//!
//!
//!
//!
//!

use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use crate::types::user::UserResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub data: UserResponse,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Expansions {
    PinnedTweetId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TweetFields {
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

#[derive(Debug, Deserialize, Serialize)]
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
pub struct UserQueryParams {
    pub expansions: Option<Expansions>,
    pub tweet_fields: Option<Vec<TweetFields>>,
    pub user_fields: Option<Vec<UserFields>>,
}

impl UserQueryParams {
    /// Construct querys params from the provided optional parameters
    pub fn construct_query_string(&self) -> String {
        let mut query = vec![];

        if let Some(expansion) = &self.expansions {
            let serialized_expansion = serde_json::to_string(expansion)
                .unwrap()
                .trim_matches('"')
                .to_string();
            query.push(format!("expansions={}", serialized_expansion));
        }

        if let Some(tweet_fields) = &self.tweet_fields {
            let fields = tweet_fields
                .iter()
                .map(|f| {
                    serde_json::to_string(f)
                        .unwrap()
                        .trim_matches('"')
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(",");
            query.push(format!("tweet.fields={}", fields));
        }

        if let Some(user_fields) = &self.user_fields {
            let fields = user_fields
                .iter()
                .map(|f| {
                    serde_json::to_string(f)
                        .unwrap()
                        .trim_matches('"')
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(",");
            query.push(format!("user.fields={}", fields));
        }
        query.join("&")
    }
}
/// Users lookup
/// API reference index

impl TweetyClient {
    /// Fetches detailed information about a user specified by their ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to retrieve information about.
    /// * `params` - Additional query parameters to include in the request, such as expansions or specific fields.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Response` with the user data if successful, or a `TweetyError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tweety_rs::api::client::TweetyClient;
    /// use tweety_rs::api::error::TweetyError;
    /// use tweety_rs::api::user::UserQueryParams;
    ///
    /// async fn example() -> Result<(), TweetyError> {
    ///     let client = TweetyClient::new("your_consumer_key","your_access_token","your_consumer_key_secret","your_access_token_secret");
    ///     let user_id = "12345";
    ///     let params = None;
    ///
    ///     let response = client.get_user_by_id(user_id, params).await?;
    ///     println!("{:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_user_by_id(
        &self,
        user_id: &str,
        params: Option<UserQueryParams>,
    ) -> Result<Value, TweetyError> {
        let query_string = if let Some(params) = params {
            params.construct_query_string()
        } else {
            String::new()
        };
        let url = format!("https://api.x.com/2/users/{}?{}", user_id, query_string);
        self.send_request::<()>(&url, Method::GET, None).await
    }

    /// <https://developer.x.com/en/docs/x-api/users/lookup/api-reference/get-users#tab1>
    /// Endpoint URL: https://api.x.com/2/users
    /// Fetches detailed information about one or more users specified by their IDs.
    pub async fn get_users(
        &self,
        ids: Vec<String>,
        params: Option<UserQueryParams>,
    ) -> Result<Value, TweetyError> {
        let ids_string = ids.join(",");
        let query_string = if let Some(params) = params {
            params.construct_query_string()
        } else {
            String::new()
        };

        let url = format!(
            "https://api.x.com/2/users?ids={}&{}",
            ids_string, query_string
        );
        self.send_request::<()>(&url, Method::GET, None).await
    }
    ///  Returns a variety of information about one or more users specified by their usernames.
    ///  Required 	string	A comma separated list of user IDs. Up to 100 are allowed in a single request.
    /// Make sure to not include a space between commas and fields.
    pub async fn get_users_by_username(&self, username: &[&str]) -> Result<Value, TweetyError> {
        let url = format!(
            "https://api.x.com/2/users/by/username/{}",
            username.join(",")
        );
        self.send_request::<()>(&url, Method::GET, None).await
    }
    /// GET /2/users/by
    /// Returns a variety of information about one or more users specified by their usernames.
    /// usernames
    ///  Required 	string A comma separated list of Twitter usernames (handles).
    /// Up to 100 are allowed in a single request. Make sure to not include a space between commas and fields.
    pub async fn get_users_by_usernames(
        &self,
        user_names: &[&str],
        params: Option<UserQueryParams>,
    ) -> Result<Value, TweetyError> {
        let query_string = if let Some(params) = params {
            params.construct_query_string()
        } else {
            String::new()
        };

        let url = format!(
            "https://api.x.com/2/users/by/?username={}&{}",
            user_names.join(","),
            query_string
        );

        self.send_request::<()>(&url, Method::GET, None).await
    }
    /// Returns information about an authorized user.
    /// <https://developer.x.com/en/docs/x-api/users/lookup/api-reference/get-users-me#>
    pub async fn get_user_me(&self, params: Option<UserQueryParams>) -> Result<Value, TweetyError> {
        let query_string = if let Some(params) = params {
            params.construct_query_string()
        } else {
            String::new()
        };
        let url = format!("https://api.x.com/2/users/me?{}", query_string);

        self.send_request::<()>(&url, Method::GET, None).await
    }
}
