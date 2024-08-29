use super::{error::TweetyError, user::UserQueryParams};
use crate::api::client::TweetyClient;
use reqwest::Method;
use serde_json::Value;

impl TweetyClient {
    /// Returns a list of users who are followers of the specified user ID.
    /// <https://developer.x.com/en/docs/x-api/users/follows/api-reference/get-users-id-followers>
    pub async fn get_user_followers(
        &self,
        user_id: &str,
        params: Option<UserQueryParams>,
    ) -> Result<Value, TweetyError> {
        let mut url = format!("https://api.x.com/2/users/{}/followers?", user_id);

        if let Some(param_str) = params {
            let query_string = param_str.construct_query_string();
            url.push_str(&query_string);
        }
        self.send_request::<()>(&url, Method::GET, None).await
    }
}
