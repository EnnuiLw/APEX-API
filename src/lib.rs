/// APEX APiI for Rust
///
/// Sanple
/// ```rs
/// use apex_api::{Client, BaseClient};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let token = "YOUR_TOKEN";
///     let client = Client::new(token);
///
///     let map = client.ranked_map().await?;
/// }
/// ```
///
///
mod client;
pub mod error;
pub mod models;
pub mod types;

// Homemade
pub use error::Error;
pub type ClientResult<T> = std::result::Result<T, Error>;

pub use client::Client;
pub use client::http::BaseClient;
pub use types::{Character, Platform};

#[cfg(feature = "env")]
pub use dotenvy;
