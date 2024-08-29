use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct FollowBody {
    target_user_id: String,
}
impl TweetyClient {
    /// Allows a user ID to follow another user.
    /// If the target user does not have public Tweets, this endpoint will send a follow request.
    /// <https://developer.x.com/en/docs/x-api/users/follows/api-reference/post-users-source_user_id-following>
    pub async fn follow_user(
        &self,
        user_id: &str,
        target_user_id: &str,
    ) -> Result<Value, TweetyError> {
        let url = format!("https://api.x.com/2/users/:{}/following", user_id);

        let json_body = FollowBody {
            target_user_id: target_user_id.to_string(),
        };

        self.send_request(&url, Method::POST, Some(json_body)).await
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
