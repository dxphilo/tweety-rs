use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde_json::Value;

impl TweetyClient {
    // Follow a user, you need to pass the username of the user to follow
    pub async fn follow_user(&self, username: &str) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/:{}/following", username);

        self.send_request::<()>(&url, Method::POST, None).await
    }
    // Allows a user ID to unfollow another user.
    // The request succeeds with no action when the authenticated user
    // sends a request to a user they're not following or have already unfollowed.
    // https://developer.x.com/en/docs/x-api/users/follows/api-reference/delete-users-source_id-following
    pub async fn unfollow_user(
        &self,
        source_userid: &str,
        target_userid: &str,
    ) -> Result<Value, TweetyError> {
        let url = format!(
            "https://api.x.com/2/users/:{}/following/:{}",
            source_userid, target_userid
        );

        self.send_request::<()>(&url, Method::DELETE, None).await
    }
}
