use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

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

impl TweetyClient {
    // GET A TWEET
    // We need to pass the tweet id we want to get its metadata information
    // GET /2/tweets
    // Returns a variety of information about the Tweet specified by the requested ID or list of IDs.
    // https://developer.x.com/en/docs/x-api/tweets/lookup/api-reference/get-tweets
    pub async fn get_tweet(&self, tweet_id: Ids) -> Result<Value, TweetyError> {
        let base_url = format!("https://api.x.com/2/tweets/?ids={}", tweet_id);

        self.send_request::<()>(&base_url, Method::GET, None).await
    }
    // GET /2/tweets/:id
    // Returns a variety of information about a single Tweet specified by the requested ID.
    // https://developer.x.com/en/docs/x-api/tweets/lookup/api-reference/get-tweets-id
    pub async fn get_tweet_info(&self, tweet_id: &str) -> Result<Value, TweetyError> {
        let base_url = format!("https://api.x.com/2/tweets/{}", tweet_id);

        self.send_request::<()>(&base_url, Method::GET, None).await
    }

    // SEND tweet message, Media id is optional for attaching tweets with an image
    // You need to uploads the image first and then pass the returned media ID here
    pub async fn post_tweet(
        &self,
        message: &str,
        media_id: Option<u64>,
    ) -> Result<Value, TweetyError> {
        let base_url = "https://api.twitter.com/2/tweets";

        let body = if let Some(id) = media_id {
            serde_json::json!({
                "text": message,
                "media_ids": id,
            })
        } else {
            serde_json::json!({
                "text": message,
            })
        };

        return self.send_request(&base_url, Method::POST, Some(body)).await;
    }
    // UPDATE/EDIT TWEET
    pub async fn edit_tweet(self, message: &str, media_id: &str) -> Result<Value, TweetyError> {
        let base_url = format!("https://api.twitter.com/2/tweets/{}", media_id);

        let body = serde_json::json!({
            "text": message,
        });

        return self
            .send_request(&base_url, Method::PATCH, Some(body))
            .await;
    }

    // DELETE TWEET
}
