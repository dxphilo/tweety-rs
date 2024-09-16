//! # tweety-rs
//!
//! **tweety-rs** is a Rust crate for interacting with the Twitter API. It provides a convenient interface for performing actions such as posting tweets, managing followers, sending direct messages, and more.
//!
//! ## Features
//!
//! - Post and edit tweets
//! - Manage followers and followings
//! - Like and retweet posts
//! - Fetch direct messages
//! - Manage bookmarks
//! - Upload media files
//! - Search tweets and users
//! - Hide replies to tweets
//!
//! ## Installation
//!
//! Run the command:
//!
//! ```bash
//! cargo add tweety-rs
//! ```
//!
//! Then, in your `main.rs` or `lib.rs`:
//!
//! ```rust
//! use tweety_rs;
//! ```
//!
//! ## Authentication
//!
//! To authenticate with the Twitter API, you will need the following credentials:
//!
//! - Consumer Key: Your application's consumer key (API key)
//! - Consumer Key Secret: Your application's consumer secret (API secret key)
//! - Access Token: Your access token
//! - Access Token Secret: Your access token secret
//!
//! You can obtain these from the [Twitter Developer portal](https://developer.x.com/en/portal/projects-and-apps).
//!
//! ## Usage
//! To get started, you'll need to create a `TweetyClient` with your Twitter API credentials.
//!
//! ### Example: How to post a tweet
//!
//! ```rust
//! use tweety_rs::client::TweetyClient;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = TweetyClient::new(
//!         "your_consumer_key",
//!         "your_consumer_key_secret",
//!         "your_access_token",
//!         "your_access_token_secret",
//!     );
//!
//!     // Post a tweet
//!     client.post_tweet("Hello, Twitter!", None).unwrap();
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Example: Tweeting with image
//!
//! Make a tweet with an image appended to your tweet.
//!
//! ```rust
//! use tweety_rs::{
//!     types::tweet::{Media, PostTweetParams},
//!     TweetyClient,
//! };
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = TweetyClient::new(
//!         "your_consumer_key",
//!         "your_consumer_key_secret",
//!         "your_access_token",
//!         "your_access_token_secret",
//!     );
//!
//!     let path = Path::new(&file_path); // path of the image to be uploaded
//!
//!     match client.upload_file(&path).await {
//!         Ok(value) => {
//!             let media_string = value.to_string();
//!             let message = format!("#{}", self.file_content.1);
//!
//!             let params = PostTweetParams {
//!                 direct_message_deep_link: None,
//!                 for_super_followers_only: None,
//!                 geo: None,
//!                 media: Some(Media {
//!                     media_ids: vec![media_string].into(),
//!                     tagged_user_ids: None,
//!                 }),
//!                 poll: None,
//!                 quote_tweet_id: None,
//!                 reply: None,
//!                 reply_settings: None,
//!             };
//!
//!             match client.post_tweet(&message, Some(params)).await {
//!                 Ok(status_code) => {
//!                     println!("Posted tweet: {:?}", status_code);
//!                 }
//!                 Err(err) => {
//!                     println!("Error posting tweet: {}", err);
//!                 }
//!             }
//!         }
//!         Err(err) => {
//!             println!("Error uploading images{}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ### Example: Retweeting
//!
//! ```rust
//! use tweety_rs::client::TweetyClient;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = TweetyClient::new(
//!         "your_consumer_key",
//!         "your_consumer_key_secret",
//!         "your_access_token",
//!         "your_access_token_secret",
//!     );
//!
//!     // Retweet a tweet by ID
//!     client.retweet("1234567890")?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Example: Get Direct Message
//!
//! Here’s an example of how to use the `get_direct_messages` function:
//!
//! ```rust
//! use tweety_rs::{client::TweetyClient,error::TweetyError,direct_messages::QueryParams};
//! use serde_json::Value;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), TweetyError> {
//!     // Create an instance of TweetyClient
//!     let client = TweetyClient::new(
//!         "your_consumer_key",
//!         "your_consumer_key_secret",
//!         "your_access_token",
//!         "your_access_token_secret",
//!     );
//!
//!     // Define query parameters
//!     let params = QueryParams {
//!         dm_event_fields: Some(vec![DMEventField::Id, DMEventField::Text]),
//!         event_types: Some(vec![EventType::MessageCreate]),
//!         expansions: Some(vec![Expansion::SenderId]),
//!         max_results: Some(50),
//!         media_fields: Some(vec![MediaField::Url, MediaField::Type]),
//!         tweet_fields: Some(vec![TweetField::CreatedAt, TweetField::Text]),
//!         user_fields: Some(vec![UserField::Username, UserField::Verified]),
//!     };
//!
//!     // Fetch direct messages
//!     match client.get_direct_messages(params).await {
//!         Ok(response) => {
//!             println!("Direct messages: {}", response);
//!         }
//!         Err(e) => {
//!             eprintln!("Error fetching direct messages: {:?}", e);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Twitter API Rate Limits
//!
//! Be aware of Twitter's rate limits:
//!
//! - **Free Access**: 50 requests every 24 hours.
//! - **Elevated Access**: 300 requests per 15-minute window per user.
//!
//! ## Modules
//!
//! - bookmark - Manage bookmarks
//! - client - Main client for interacting with the Twitter API
//! - direct_messages - Handle sending and receiving direct messages
//! - favourites - Manage favourites (likes)
//! - followers - Manage followers
//! - following - Manage followings
//! - hide_replies - Hide replies to tweets
//! - like - Like tweets
//! - mentions - Manage mentions
//! - retweets - Retweet tweets
//! - search - Search tweets and users
//! - tweet - Post and manage tweets
//! - uploads - Upload media files
//! - user - Manage user information
//!
//! ## Common Issues
//!
//! - **Authentication Issues**: Make sure your credentials are correct.
//! - **OAuth1 Permissions**: Ensure your app has the appropriate OAuth1 permissions.
//!
//! ## Bots using this Crate
//! - [County Flags](https://twitter.com/DailyPexels)
//! - [Kenya Shilling Rates](https://x.com/kshsrates)
//! - [AI Bot](https://x.com/ph
//! ## Contributing
//! Contributions are welcome! Submit a pull request or open an issue on GitHub.
//!
//! ## License
//! This project is licensed under the MIT License.
//!
//! ## Acknowledgments
//! This project was inspired by the desire to simplify interactions with Twitter's API in a Rust-friendly way.
//!
//! If you find it useful, consider giving it a star!
pub mod api;
pub mod types;

pub use api::client::TweetyClient;
