//! # Tweety Client Library
//!
//! This library is a labor of love, designed to make your interaction
//! with Twitter as seamless as possible. Whether you're tweeting about
//! your latest project, retweeting insightful posts, or just liking
//! cat videos, this client has got you covered.
//!
//! # Why Tweety?
//!
//! Because Twitter is better when you have a reliable sidekick!
//!
//! ## Example
//!
//! ```rust
//! use tweety_rs::api::client::TweetyClient;
//!
//! let client = TweetyClient::new(
//!     "your_consumer_key",
//!    "your_access_token",
//!     "your_consumer_key_secret",
//!     "your_access_token_secret",
//! );
//!
//! assert!(client.is_initialized());
//! ```
//!
//! The `is_initialized` method checks if the client has been properly initialized
//! with all the credentials.
//!
//! # Final Thoughts
//!
//! Remember, with great power comes great responsibility.
//! Use this library wisely and make the world a better place, one tweet at a time.
//! `TweetyClient` is a client for interacting with Twitter's API. It requires OAuth
//! credentials for initialization.
//!
pub mod api;
pub mod types;

pub use api::client::TweetyClient;
