use dotenvy::dotenv;
use std::env;
use tweety_rs::api::direct_messages::*;
use tweety_rs::api::user::{Expansions, TweetFields, UserFields, UserQueryParams};
use tweety_rs::types::tweet::PostTweetParams;
use tweety_rs::TweetyClient;

fn setup_client() -> TweetyClient {
    dotenv().expect(".env file not found");

    let consumer_key = env::var("CONSUMER_API_KEY").expect("CONSUMER_API_KEY not set");
    let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN not set");
    let consumer_secret = env::var("CONSUMER_API_SECRET").expect("CONSUMER_API_SECRET not set");
    let access_secret = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET not set");

    TweetyClient::new(
        &consumer_key,
        &access_token,
        &consumer_secret,
        &access_secret,
    )
}

#[cfg(test)]
mod tests {
    use tweety_rs::api::direct_messages::QueryParams;

    use super::*;

    #[tokio::test]
    async fn test_get_user_by_id() {
        let client = setup_client();
        let response_user_id = client.get_user_by_id("2244994945", None).await;
        assert!(
            response_user_id.is_ok(),
            "Request failed{:?}",
            response_user_id
        );
    }

    #[tokio::test]
    async fn test_get_user_me() {
        let client = setup_client();
        let params = UserQueryParams {
            expansions: Some(Expansions::PinnedTweetId),
            tweet_fields: Some(vec![
                TweetFields::CreatedAt,
                TweetFields::Text,
                TweetFields::PublicMetrics,
            ]),
            user_fields: Some(vec![
                UserFields::Name,
                UserFields::Username,
                UserFields::ProfileImageUrl,
            ]),
        };

        let response = client.get_user_me(Some(params)).await;
        assert!(response.is_ok(), "Request failed {:?}", response);
    }

    /// test post_tweet
    #[tokio::test]
    async fn test_post_a_tweet() {
        let client = setup_client();
        let message = String::from("Hello testing tweety-rs in the attic");
        let body_param = PostTweetParams {
            direct_message_deep_link: None,
            for_super_followers_only: None,
            geo: None,
            media: None,
            poll: None,
            quote_tweet_id: None,
            reply: None,
            reply_settings: None,
        };

        let response = client.post_tweet(&message, Some(body_param)).await;
        assert!(response.is_ok(), "Request failed {:?}", response);
    }

    /// test get tweet info
    #[tokio::test]
    async fn test_get_tweet_info() {
        let client = setup_client();
        let response = client.get_tweet_info("1828783668107559176").await;
        assert!(response.is_ok(), "Request failed {:?}", response);
    }
    /// test get dm messages
    #[tokio::test]
    async fn test_get_dm_messages() {
        let client = setup_client();

        let params = QueryParams {
            dm_event_fields: Some(vec![DMEventField::Id, DMEventField::Text]),
            event_types: Some(vec![EventType::MessageCreate]),
            expansions: Some(vec![Expansion::SenderId]),
            max_results: Some(50),
            media_fields: Some(vec![MediaField::Url, MediaField::Type]),
            pagination_token: Some("pagination123".to_string()),
            tweet_fields: Some(vec![TweetField::CreatedAt, TweetField::Text]),
            user_fields: Some(vec![UserField::Username, UserField::Verified]),
        };

        let response_user_id = client.get_direct_messages(params).await;
        assert!(
            response_user_id.is_ok(),
            "Request failed{:?}",
            response_user_id
        );
    }
}
