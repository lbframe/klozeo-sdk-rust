/// An error returned by the Klozeo API with a structured body.
#[derive(Debug)]
pub struct ApiError {
    /// HTTP status code of the response.
    pub status_code: u16,
    /// Human-readable error message from the API.
    pub message: String,
    /// Machine-readable error code (e.g. `"rate_limit_exceeded"`).
    pub code: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {} — {} ({})", self.status_code, self.message, self.code)
    }
}

impl std::error::Error for ApiError {}

/// All errors that can be returned by the Klozeo SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The requested resource does not exist (HTTP 404).
    #[error("resource not found")]
    NotFound,

    /// The API key is missing or invalid (HTTP 401).
    #[error("unauthorized — check your API key")]
    Unauthorized,

    /// The account has reached its leads limit; upgrade required (HTTP 403).
    #[error("forbidden — leads limit reached")]
    Forbidden,

    /// The account has hit its rate limit. Wait `retry_after` seconds before retrying.
    #[error("rate limited")]
    RateLimited {
        /// Seconds to wait before the next request.
        retry_after: u64,
    },

    /// The request was malformed (HTTP 400).
    #[error("bad request: {0}")]
    BadRequest(String),

    /// A non-400/401/403/404/429 API error with status code and message.
    #[error("API error: {0}")]
    Api(#[from] ApiError),

    /// An underlying network/transport error from `reqwest`.
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// A JSON serialization or deserialization error.
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
