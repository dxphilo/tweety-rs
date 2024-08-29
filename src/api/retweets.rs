use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde_json::Value;

/// Check required params
/// <https://developer.x.com/en/docs/x-api/tweets/retweets/api-reference/get-tweets-id-retweets#tab0>
#[derive(Default)]
pub struct RetweetQueryParams {
    pub expansions: Option<String>,
    pub max_results: Option<u32>,
    pub media_fields: Option<String>,
    pub place_fields: Option<String>,
    pub poll_fields: Option<String>,
    pub tweet_fields: Option<String>,
    pub user_fields: Option<String>,
}

impl RetweetQueryParams {
    pub fn to_query_string(&self) -> String {
        let mut params = vec![];

        if let Some(ref expansions) = self.expansions {
            params.push(format!("expansions={}", expansions));
        }
        if let Some(max_results) = self.max_results {
            params.push(format!("max_results={}", max_results));
        }
        if let Some(ref media_fields) = self.media_fields {
            params.push(format!("media.fields={}", media_fields));
        }
        if let Some(ref place_fields) = self.place_fields {
            params.push(format!("place.fields={}", place_fields));
        }
        if let Some(ref poll_fields) = self.poll_fields {
            params.push(format!("poll.fields={}", poll_fields));
        }
        if let Some(ref tweet_fields) = self.tweet_fields {
            params.push(format!("tweet.fields={}", tweet_fields));
        }
        if let Some(ref user_fields) = self.user_fields {
            params.push(format!("user.fields={}", user_fields));
        }

        if !params.is_empty() {
            return params.join("&");
        }

        String::new()
    }
}

/// Retweets
/// API reference index
/// For the complete API reference, select an endpoint from the list:
/// <https://developer.x.com/en/docs/x-api/tweets/retweets/api-reference>

impl TweetyClient {
    /// Users who have Retweeted a Post
    pub async fn fetch_retweeters(self, tweet_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/tweets/{}/retweeted_by", tweet_id);
        self.send_request::<()>(&url, Method::GET, None).await
    }
    /// Causes the user ID identified in the path parameter to Retweet the target Tweet.
    pub async fn retweet(&self, tweet_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/{}/retweets", tweet_id);
        self.send_request::<()>(&url, Method::POST, None).await
    }
    /// Allows a user or authenticated user ID to remove the Retweet of a Tweet.
    /// The request succeeds with no action when the user sends a request to a user
    /// they're not Retweeting the Tweet or have already removed the Retweet of.
    /// <https://developer.x.com/en/docs/x-api/tweets/retweets/api-reference/delete-users-id-retweets-tweet_id>
    pub async fn unretweet(
        &self,
        user_id: &str,
        source_tweet_id: &str,
    ) -> Result<Value, TweetyError> {
        let url = format!(
            "https://api.x.com/2/users/{}/retweets/{}",
            user_id, source_tweet_id
        );
        self.send_request::<()>(&url, Method::DELETE, None).await
    }

    /// ## Query Parameters
    ///
    /// <https://developer.x.com/en/docs/x-api/tweets/retweets/api-reference/get-tweets-id-retweets>

    /// Returns the Retweets for a given Tweet ID.
    pub async fn fetch_retweets(
        self,
        user_id: &str,
        params: Option<RetweetQueryParams>,
    ) -> Result<Value, TweetyError> {
        let mut url = format!("https://api.x.com/2/tweets/{}/retweets", user_id);

        if let Some(query_params) = params {
            let query_string = query_params.to_query_string();
            if !query_string.is_empty() {
                url.push_str("?");
                url.push_str(&query_params.to_query_string());
            }
        }
        self.send_request::<()>(&url, Method::GET, None).await
    }
}
