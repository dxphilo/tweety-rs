use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum TweetyError {
    #[error("Error reading file: {0}")]
    FileIOError(String),
    #[error("Connection error occurred: {0}")]
    NetworkError(String),
    #[error("Authentication error occurred")]
    AuthError,
    #[error("Request Error :{0}")]
    RequestError(reqwest::Error),
    #[error("API responded with an error: {0}")]
    ApiError(String),
    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(String),
    #[error("Parse Error: {0}")]
    UrlParseError(ParseError),
    #[error("Twitter consumer credentials are missing. Please set the 'consumer_key', 'consumer_secret', 'access_token', and 'access_token_secret' in your configuration:")]
    MissingCredentials,
}

impl From<reqwest::Error> for TweetyError {
    fn from(err: reqwest::Error) -> TweetyError {
        TweetyError::RequestError(err)
    }
}
