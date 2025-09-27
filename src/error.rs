#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// for HTTP Client
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// for serde
    #[error(transparent)]
    Serialize(#[from] serde_json::Error),
    /// for Cache
    #[error("Cache Error:\n\t{0}")]
    Cache(String),
    /// for Cooldown
    #[error("Now cooldown. Please after {0} {1}")]
    Cooldown(i64, String),
    /// for API
    #[error("{0}")]
    API(String),
    /// Dotenvt
    #[cfg(feature = "env")]
    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
}
