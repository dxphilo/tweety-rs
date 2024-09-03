use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTweetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_deep_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_super_followers_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tweet_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<Reply>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<String>,
}

impl PostTweetParams {
    /// takes the message and appends it to the body json string
    pub fn to_json(&self, message: &str) -> String {
        let mut json_payload = serde_json::to_value(&self).unwrap();
        json_payload["text"] = serde_json::Value::String(message.to_string());

        serde_json::to_string(&json_payload).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged_user_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_tweet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_reply_user_ids: Option<Vec<String>>,
}
