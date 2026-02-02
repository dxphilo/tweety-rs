use std::collections::HashMap;

use crate::api::error::TweetyError;
use crate::types::types::ResponseWithHeaders;
use reqwest::{Client, Method};
use reqwest_oauth1::{self, OAuthClientProvider};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetyClient {
    pub(crate) consumer_key: String,
    pub(crate) access_token: String,
    pub(crate) consumer_key_secret: String,
    pub(crate) access_token_secret: String,
}

impl TweetyClient {
    /// Creates a new `TweetyClient` instance with the given credentials.
    ///
    /// # Parameters
    ///
    /// * `consumer_key` - The OAuth consumer key.
    /// * `access_token` - The OAuth access token.
    /// * `consumer_key_secret` - The OAuth consumer key secret.
    /// * `access_token_secret` - The OAuth access token secret.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tweety_rs::api::client::TweetyClient;
    /// let client = TweetyClient::new(
    ///     "your_consumer_key",
    ///     "your_access_token",
    ///     "your_consumer_key_secret",
    ///     "your_access_token_secret",
    /// );
    /// ```
    pub fn new(
        consumer_key: &str,
        access_token: &str,
        consumer_key_secret: &str,
        access_token_secret: &str,
    ) -> Self {
        TweetyClient {
            consumer_key: consumer_key.to_string(),
            access_token: access_token.to_string(),
            consumer_key_secret: consumer_key_secret.to_string(),
            access_token_secret: access_token_secret.to_string(),
        }
    }
    pub fn is_initialized(&self) -> bool {
        !self.consumer_key.is_empty()
            && !self.access_token.is_empty()
            && !self.consumer_key_secret.is_empty()
            && !self.access_token_secret.is_empty()
    }
    pub(crate) async fn send_request<T>(
        &self,
        url: &str,
        method: Method,
        body: Option<T>,
    ) -> Result<ResponseWithHeaders, TweetyError>
    where
        T: Serialize + Deserialize<'static>,
    {
        if !self.is_initialized() {
            return Err(TweetyError::MissingCredentials);
        };

        let parsed_url = match Url::parse(url) {
            Ok(url) => url.to_string(),
            Err(err) => {
                return Err(TweetyError::UrlParseError(err));
            }
        };

        let secrets = reqwest_oauth1::Secrets::new(&self.consumer_key, &self.consumer_key_secret)
            .token(&self.access_token, &self.access_token_secret);

        let client = Client::new();
        let mut json_body = String::new();

        if body.is_some() {
            json_body = serde_json::to_string(&body).unwrap();
        }

        let response = match method {
            Method::POST => client
                .oauth1(secrets)
                .post(&parsed_url)
                .header("Content-Type", "application/json")
                .body(json_body)
                .send(),
            Method::GET => client.oauth1(secrets).get(&parsed_url).send(),
            Method::DELETE => client.oauth1(secrets).delete(&parsed_url).send(),
            Method::PUT => client.oauth1(secrets).put(&parsed_url).send(),
            _ => panic!("Method not allowed"),
        };

        match response.await {
            Ok(response) => {
                let headers = response
                    .headers()
                    .clone()
                    .iter()
                    .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
                    .collect::<HashMap<String, String>>();

                if response.status().is_success() {
                    let api_response = response
                        .json::<Value>()
                        .await
                        .map_err(|err| TweetyError::JsonParseError(err.to_string()))?;

                    // Returning of headers is related to this issue: https://github.com/dxphilo/tweety-rs/issues/2
                    let res = ResponseWithHeaders {
                        body: api_response,
                        headers,
                    };

                    return Ok(res);
                }
                let status = response.status();

                let status_text = response
                    .json::<Value>()
                    .await
                    .map_err(|err| TweetyError::JsonParseError(err.to_string()))?;

                Err(TweetyError::ApiError(format!(
                    "HTTP {}: Status Text: {}: Headers: {:?}",
                    status, status_text, headers
                )))
            }
            Err(err) => Err(TweetyError::NetworkError(err.to_string())),
        }
    }
}
