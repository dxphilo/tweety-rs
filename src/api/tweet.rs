use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use crate::types::tweet::PostTweetParams;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRequest {
    pub text: String,
    pub media: Media,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub media_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub data: DeleteData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteData {
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ids {
    Single(String),
    Multiple(Vec<String>),
}

impl fmt::Display for Ids {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ids::Single(id) => write!(f, "{}", id),
            Ids::Multiple(ids) => write!(f, "{}", ids.join(",")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTweetResponseData {
    pub data: TweetResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetResponse {
    pub edit_history_tweet_ids: Vec<String>,
    pub id: String,
    pub text: String,
}

impl TweetyClient {
    /// GET A TWEET
    /// We need to pass the tweet id we want to get its metadata information
    /// GET /2/tweets
    /// Returns a variety of information about the Tweet specified by the requested ID or list of IDs.
    /// [Docs](https://developer.x.com/en/docs/x-api/tweets/lookup/api-reference/get-tweets)
    pub async fn get_tweet(&self, tweet_id: Ids) -> Result<Value, TweetyError> {
        let base_url = format!("https://api.x.com/2/tweets/?ids={}", tweet_id);

        self.send_request::<()>(&base_url, Method::GET, None).await
    }
    /// GET /2/tweets/:id
    /// Returns a variety of information about a single Tweet specified by the requested ID.
    /// [Docs](https://developer.x.com/en/docs/x-api/tweets/lookup/api-reference/get-tweets-id)
    pub async fn get_tweet_info(&self, tweet_id: &str) -> Result<Value, TweetyError> {
        let base_url = format!("https://api.x.com/2/tweets/{}", tweet_id);

        self.send_request::<()>(&base_url, Method::GET, None).await
    }

    /// SEND tweet message, Media id is optional for attaching tweets with an image
    /// You need to uploads the image first and then pass the returned media ID here
    /// [Docs](https://developer.x.com/en/docs/x-api/tweets/manage-tweets/api-reference/post-tweets)
    pub async fn post_tweet(
        &self,
        message: &str,
        body_params: Option<PostTweetParams>,
    ) -> Result<PostTweetResponseData, TweetyError> {
        let base_url = "https://api.twitter.com/2/tweets";

        let json_body = if let Some(body) = body_params {
            body.to_json(message)
        } else {
            let json_data = serde_json::json!({ "text": message });
            json_data
        };

        match self
            .send_request(base_url, Method::POST, Some(json_body))
            .await
        {
            Ok(value) => match serde_json::from_value::<PostTweetResponseData>(value) {
                Ok(res) => Ok(res),
                Err(e) => Err(TweetyError::JsonParseError(e.to_string())),
            },
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
    /// UPDATE/EDIT TWEET
    pub async fn edit_tweet(self, message: &str, media_id: &str) -> Result<Value, TweetyError> {
        let base_url = format!("https://api.twitter.com/2/tweets/{}", media_id);

        let body = serde_json::json!({
            "text": message,
        });

        self.send_request(&base_url, Method::PATCH, Some(body))
            .await
    }

    /// DELETE TWEET
    /// Path parameter, pass The Tweet ID you are deleting.
    /// [Delete Docs](https://developer.x.com/en/docs/x-api/tweets/manage-tweets/api-reference/delete-tweets-id)
    pub async fn delete_tweet(&self, tweet_id: &str) -> Result<DeleteResponse, TweetyError> {
        let url = format!("https://api.x.com/2/tweets/{}", tweet_id);

        match self.send_request::<()>(&url, Method::DELETE, None).await {
            Ok(value) => match serde_json::from_value::<DeleteResponse>(value) {
                Ok(res) => Ok(res),
                Err(err) => Err(TweetyError::JsonParseError(err.to_string())),
            },
            Err(err) => Err(err),
        }
    }
}
