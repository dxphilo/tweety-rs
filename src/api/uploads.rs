use crate::api::client::TweetyClient;
use crate::api::error::TweetyError;
use reqwest::multipart;
use reqwest_oauth1::OAuthClientProvider;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Media {
    pub media_id: u64,
}

impl TweetyClient {
    /// Create a media from a file<br/>
    /// Will fail if `consumer_key`, `consumer_secret`, `access_token` and `access_token_secret` are not set
    /// Will take a path as a parameter and return the media id of the uploadeded file is sucess and TweetyError incase of failure
    pub async fn upload_file(&self, path: &Path) -> Result<u64, TweetyError> {
        if !self.is_initialized() {
            return Err(TweetyError::MissingCredentials);
        }

        let mut file = match File::open(path) {
            Ok(value) => value,
            Err(err) => {
                println!("Error opening file: {:?}", err);
                return Err(TweetyError::FileIOError(err.to_string()));
            }
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let part = multipart::Part::bytes(buffer)
            .file_name(path.file_name().unwrap().to_string_lossy().to_string());

        let form = multipart::Form::new().part("media", part);

        let secrets = reqwest_oauth1::Secrets::new(&self.consumer_key, &self.consumer_key_secret)
            .token(&self.access_token, &self.access_token_secret);
        let client = reqwest::Client::new();

        let response = client
            .oauth1(secrets)
            .post("https://upload.twitter.com/1.1/media/upload.json")
            .multipart(form)
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let res = res.json::<Media>().await?;
                    return Ok(res.media_id);
                } else {
                    let err = res.json().await;
                    if let Ok(value) = err {
                        return Ok(value);
                    }
                    return Err(TweetyError::JsonParseError(
                        "Error parsing multipart JSON".to_string(),
                    ));
                }
            }
            Err(err) => {
                return Err(TweetyError::NetworkError(err.to_string()));
            }
        }
    }
}
