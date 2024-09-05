use super::{error::TweetyError, user::UserQueryParams};
use crate::api::client::TweetyClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowersResponse {
    pub data: Vec<Follower>,
    pub meta: MetaData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Follower {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub result_count: u32,
    pub next_token: String,
}

impl TweetyClient {
    /// Returns a list of users who are followers of the specified user ID.
    /// [Docs](https://developer.x.com/en/docs/x-api/users/follows/api-reference/get-users-id-followers)
    pub async fn get_user_followers(
        &self,
        user_id: &str,
        params: Option<UserQueryParams>,
    ) -> Result<UserFollowersResponse, TweetyError> {
        let mut url = format!("https://api.x.com/2/users/{}/followers?", user_id);

        if let Some(param_str) = params {
            let query_string = param_str.construct_query_string();
            url.push_str(&query_string);
        }
        match self.send_request::<()>(&url, Method::GET, None).await {
            Ok(value) => match serde_json::from_value::<UserFollowersResponse>(value) {
                Ok(data) => Ok(data),
                Err(err) => Err(TweetyError::JsonParseError(err.to_string())),
            },
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
}
