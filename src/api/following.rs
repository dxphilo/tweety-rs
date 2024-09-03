use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use super::user::UserQueryParams;

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowResponse {
    pub data: FollowData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowData {
    pub following: bool,
    pub pending_follow: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FollowBody {
    target_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfollowResponse {
    pub data: UnfollowData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfollowData {
    pub following: bool,
}

// USER FOLLOWING STRUCT
#[derive(Serialize, Deserialize, Debug)]
pub struct UserFollowingResponse {
    pub data: Vec<User>,
    pub includes: Includes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub pinned_tweet_id: Option<String>,
    pub id: String,
    pub username: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Includes {
    pub tweets: Vec<Tweet>,
    pub meta: MetaData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    pub context_annotations: Option<Vec<ContextAnnotation>>,
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextAnnotation {
    pub domain: Domain,
    pub entity: Entity,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub result_count: u32,
    pub next_token: String,
}

impl TweetyClient {
    /// Allows a user ID to follow another user.
    /// If the target user does not have public Tweets, this endpoint will send a follow request.
    /// [Docs](https://developer.x.com/en/docs/x-api/users/follows/api-reference/post-users-source_user_id-following)
    pub async fn follow_user(
        &self,
        user_id: &str,
        target_user_id: &str,
    ) -> Result<FollowResponse, TweetyError> {
        let url = format!("https://api.x.com/2/users/:{}/following", user_id);

        let json_body = FollowBody {
            target_user_id: target_user_id.to_string(),
        };

        match self.send_request(&url, Method::POST, Some(json_body)).await {
            Ok(value) => match serde_json::from_value::<FollowResponse>(value) {
                Ok(data) => Ok(data),
                Err(err) => Err(TweetyError::JsonParseError(err.to_string())),
            },
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
    /// Allows a user ID to unfollow another user.
    /// The request succeeds with no action when the authenticated user
    /// sends a request to a user they're not following or have already unfollowed.
    /// [Docs](https://developer.x.com/en/docs/x-api/users/follows/api-reference/delete-users-source_id-following)
    pub async fn unfollow_user(
        &self,
        source_userid: &str,
        target_userid: &str,
    ) -> Result<UnfollowResponse, TweetyError> {
        let url = format!(
            "https://api.x.com/2/users/:{}/following/:{}",
            source_userid, target_userid
        );

        match self.send_request::<()>(&url, Method::DELETE, None).await {
            Ok(value) => match serde_json::from_value::<UnfollowResponse>(value) {
                Ok(data) => Ok(data),
                Err(err) => Err(TweetyError::JsonParseError(err.to_string())),
            },
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
    pub async fn get_users_following(
        &self,
        user_id: &str,
        query: Option<UserQueryParams>,
    ) -> Result<UserFollowingResponse, TweetyError> {
        let mut base_url = format!("https://api.x.com/2/users/{}/following", user_id);

        if let Some(query) = query {
            let query_string = query.construct_query_string();
            base_url.push_str(&query_string);
        }

        match self.send_request::<()>(&base_url, Method::GET, None).await {
            Ok(value) => match serde_json::from_value::<UserFollowingResponse>(value) {
                Ok(data) => Ok(data),
                Err(err) => Err(TweetyError::JsonParseError(err.to_string())),
            },
            Err(err) => Err(TweetyError::ApiError(err.to_string())),
        }
    }
}
