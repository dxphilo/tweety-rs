use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde_json::Value;

impl TweetyClient {
    /// DELETE https://api.x.com/2/tweets/:id/likes/:tweet_id
    /// (unlike a Post)
    /// https://developer.x.com/en/docs/x-api/tweets/likes/migrate/manage-likes-standard-to-twitter-api-v2
    pub async fn unlike_tweet(&self, user_id: u64, tweet_id: u64) -> Result<Value, TweetyError> {
        let url = format!(
            "https://api.x.com/2/tweets/:{}/likes/:{}",
            user_id, tweet_id
        );

        self.send_request::<()>(&url, Method::DELETE, None).await
    }
    /// Users who have liked a Post
    /// https://developer.x.com/en/docs/x-api/tweets/likes/api-reference
    pub async fn get_users_who_liked_a_post(&self, post_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/tweets/{}/liking_users", post_id);

        self.send_request::<()>(&url, Method::GET, None).await
    }

    /// Posts liked by a user
    /// https://developer.x.com/en/docs/x-api/tweets/likes/api-reference
    pub async fn get_posts_liked_by_a_user(&self, user_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/{}/liked_tweets", user_id);

        self.send_request::<()>(&url, Method::GET, None).await
    }
    /// MANAGE LIKES

    /// Allows a user ID to like a Post
    /// https://developer.x.com/en/docs/x-api/tweets/likes/api-reference
    pub async fn like_a_post(&self, user_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/{}/likes", user_id);

        self.send_request::<()>(&url, Method::POST, None).await
    }

    /// Allows a user ID to unlike a Post
    /// https://developer.x.com/en/docs/x-api/tweets/likes/api-reference
    pub async fn unlike_a_post(&self, user_id: &str, tweet_id: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/{}/likes/{}", user_id, tweet_id);

        self.send_request::<()>(&url, Method::DELETE, None).await
    }
}
