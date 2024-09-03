use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HideTweet {
    hidden: bool,
}
/// REFERENCE LINK
/// Hide replies
/// PUT /2/tweets/:id/hidden
/// Hides or unhides a reply to a Tweet.
/// Endpoint URL
/// https://api.x.com/2/tweets/:id/hidden
/// hidden	boolean	Indicates if the Tweet was successfully hidden or unhidden.
impl TweetyClient {
    pub async fn hide_tweet(self, tweet_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/tweets/{}/hidden", tweet_id);
        let json_body = HideTweet { hidden: true };
        self.send_request(&url, Method::PUT, Some(json_body)).await
    }
}
