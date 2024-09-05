# Tweety-rs

[![Crates.io](https://img.shields.io/crates/v/tweety-rs.svg)](https://crates.io/crates/tweety-rs)
[![Docs.rs](https://docs.rs/tweety-rs/badge.svg)](https://docs.rs/tweety-rs)
[![License](https://img.shields.io/crates/l/tweety-rs.svg)](https://github.com/dxphilo/tweety-rs/blob/main/LICENSE)

**Tweety-rs** is a Rust crate for interacting with the Twitter API. It provides a convenient interface for performing actions such as posting tweets, managing followers, sending direct messages, and more.

## Features

- Post and edit tweets
- Manage followers and followings
- Like and retweet posts
- Fetch direct messages
- Manage bookmarks
- Upload media files
- Search tweets and users
- Hide replies to tweets

## Installation

Add `tweety-rs` to your `Cargo.toml`:

```toml
[dependencies]
tweety-rs = "0.1.2"
```
#### Alternatively

```
cargo add tweety-rs
```

Then, in your main.rs or lib.rs:

```rust
use crate tweety_rs;
```
## Usage
To get started, you'll need to create a TweetyClient with your Twitter API credentials. Here's an example of how to post a tweet:

```rust
use tweety_rs::client::TweetyClient;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TweetyClient::new(
        "your_consumer_key",
        "your_consumer_key_secret",
        "your_access_token",
        "your_access_token_secret",
    );

    // Post a tweet
    client.post_tweet("Hello, Twitter!")?;

    Ok(())
}
```
## Authentication

To authenticate with the Twitter API, you will need the following credentials:

- Consumer Key: Your application's consumer key (API key)
- Consumer Key Secret: Your application's consumer secret (API secret key)
- Access Token: Your access token
- Access Token Secret: Your access token secret

You can obtain these from the [Twitter Developer portal](https://developer.x.com/en/portal/projects-and-apps).

### Example: Retweeting

```rust
use tweety_rs::client::TweetyClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TweetyClient::new(
        "your_consumer_key",
        "your_consumer_key_secret",
        "your_access_token",
        "your_access_token_secret",
    );

    // Retweet a tweet by ID
    client.retweet("1234567890")?;

    Ok(())
}
```

### Example: Get Direct Message

Here’s an example of how to use the `get_direct_messages` function:

```rust
use tweety_rs::{client::TweetyClient,error::TweetyError,direct_messages::QueryParams};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), TweetyError> {
    // Create an instance of TweetyClient
    let client = TweetyClient::new(
        "your_consumer_key",
        "your_consumer_key_secret",
        "your_access_token",
        "your_access_token_secret",
    );

    // Define query parameters
    let params = QueryParams {
        dm_event_fields: Some(vec![DMEventField::Id, DMEventField::Text]),
        event_types: Some(vec![EventType::MessageCreate]),
        expansions: Some(vec![Expansion::SenderId]),
        max_results: Some(50),
        media_fields: Some(vec![MediaField::Url, MediaField::Type]),
        tweet_fields: Some(vec![TweetField::CreatedAt, TweetField::Text]),
        user_fields: Some(vec![UserField::Username, UserField::Verified]),
    };

    // Fetch direct messages
    match client.get_direct_messages(params).await {
        Ok(response) => {
            println!("Direct messages: {}", response);
        }
        Err(e) => {
            eprintln!("Error fetching direct messages: {:?}", e);
        }
    }

    Ok(())
}
```

# ⚠️ 🕒 Twitter API Rate Limits

Twitter has a small window cap for the free tier, so it's important to be aware of the rate limits.


  - **Elevated Access**: 300 requests per 15-minute window per user.

Check out the [rate limits Documentation](https://developer.x.com/en/docs/x-api/rate-limits)

## Modules
The crate is organized into several modules, each responsible for different aspects of the Twitter API:

- bookmark - Manage bookmarks
- client - Main client for interacting with the Twitter API
- direct_messages - Handle sending and receiving direct messages
- favourites - Manage favourites (likes)
- followers - Manage followers
- following - Manage followings
- hide_replies - Hide replies to tweets
- like - Like tweets
- mentions - Manage mentions
- retweets - Retweet tweets
- search - Search tweets and users
- tweet - Post and manage tweets
- uploads - Upload media files
- user - Manage user information

### Command Issues

- **Authentication Issues**: When authenticating requests to the Twitter API v2 endpoints, you must use keys and tokens from a Twitter developer App that is attached to a Project. You can create a project via the [Twitter Developer Portal](https://developer.twitter.com/en/portal/dashboard).

- **OAuth1 Permissions**: Your client app might not be configured with the appropriate OAuth1 app permissions for this endpoint.

### Error Resolution

- **Endpoint Access**: Some errors may indicate that you need to upgrade to the premium tier to access certain endpoints.



## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue on GitHub.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments
This project is inspired by the desire to make interacting with Twitter's API easier and more Rust-idiomatic.
