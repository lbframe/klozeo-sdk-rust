use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::errors::{ApiError, Error};
use crate::resources::leads::LeadsResource;
use crate::resources::notes::NotesResource;
use crate::resources::scoring::ScoringResource;
use crate::resources::webhooks::WebhooksResource;
use crate::types::RateLimitState;

/// Default base URL for the Klozeo API.
const DEFAULT_BASE_URL: &str = "https://app.klozeo.com/api/v1";

// ─── ClientConfig ──────────────────────────────────────────────────────────

/// Configuration options for the Klozeo [`Client`].
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API base URL. Defaults to `https://app.klozeo.com/api/v1`.
    pub base_url: String,
    /// Per-request timeout.
    pub timeout: Duration,
    /// Maximum number of retries on 429 / 5xx responses.
    pub max_retries: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_owned(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

impl ClientConfig {
    /// Returns a builder for `ClientConfig`.
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }
}

/// Builder for [`ClientConfig`].
#[derive(Default)]
pub struct ClientConfigBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl ClientConfigBuilder {
    /// Override the API base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }
    /// Set the per-request timeout.
    pub fn timeout(mut self, d: Duration) -> Self {
        self.timeout = Some(d);
        self
    }
    /// Set the maximum number of retries on 429 / 5xx.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = Some(n);
        self
    }
    /// Build the [`ClientConfig`].
    pub fn build(self) -> ClientConfig {
        let defaults = ClientConfig::default();
        ClientConfig {
            base_url: self.base_url.unwrap_or(defaults.base_url),
            timeout: self.timeout.unwrap_or(defaults.timeout),
            max_retries: self.max_retries.unwrap_or(defaults.max_retries),
        }
    }
}

// ─── ClientInner ───────────────────────────────────────────────────────────

pub(crate) struct ClientInner {
    pub(crate) http: reqwest::Client,
    pub(crate) config: ClientConfig,
    pub(crate) rate_limit: Mutex<Option<RateLimitState>>,
}

// ─── Client ────────────────────────────────────────────────────────────────

/// The main entry point for the Klozeo API.
///
/// ```rust,ignore
/// use klozeo::Client;
///
/// let client = Client::new("sk_live_your_api_key");
/// let lead = client.leads().get("cl_01234567-...").await?;
/// ```
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientInner>,
}

impl Client {
    /// Create a client with default configuration.
    ///
    /// ```rust,ignore
    /// let client = Client::new("sk_live_your_api_key");
    /// ```
    pub fn new(api_key: &str) -> Self {
        Self::with_config(api_key, ClientConfig::default())
    }

    /// Create a client with custom configuration.
    ///
    /// ```rust,ignore
    /// use klozeo::{Client, ClientConfig};
    /// use std::time::Duration;
    ///
    /// let client = Client::with_config(
    ///     "sk_live_your_api_key",
    ///     ClientConfig::builder()
    ///         .base_url("https://custom.api.com")
    ///         .timeout(Duration::from_secs(60))
    ///         .max_retries(5)
    ///         .build(),
    /// );
    /// ```
    pub fn with_config(api_key: &str, config: ClientConfig) -> Self {
        let mut headers = HeaderMap::new();
        let key_value = HeaderValue::from_str(api_key)
            .expect("API key must be a valid header value");
        headers.insert("X-API-Key", key_value);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .expect("failed to build reqwest client");

        Self {
            inner: Arc::new(ClientInner {
                http,
                config,
                rate_limit: Mutex::new(None),
            }),
        }
    }

    /// Access the leads resource.
    pub fn leads(&self) -> LeadsResource {
        LeadsResource::new(Arc::clone(&self.inner))
    }

    /// Access the notes resource.
    pub fn notes(&self) -> NotesResource {
        NotesResource::new(Arc::clone(&self.inner))
    }

    /// Access the scoring-rules resource.
    pub fn scoring(&self) -> ScoringResource {
        ScoringResource::new(Arc::clone(&self.inner))
    }

    /// Access the webhooks resource.
    pub fn webhooks(&self) -> WebhooksResource {
        WebhooksResource::new(Arc::clone(&self.inner))
    }

    /// Return the most recently observed rate-limit state, if any.
    pub fn rate_limit_state(&self) -> Option<RateLimitState> {
        self.inner.rate_limit.lock().unwrap().clone()
    }
}

// ─── Shared request helper ─────────────────────────────────────────────────

/// Parse an API error response body, falling back to a generic message.
async fn parse_api_error(status: u16, resp: reqwest::Response) -> Error {
    #[derive(serde::Deserialize)]
    struct ApiErrorBody {
        message: Option<String>,
        code: Option<String>,
        error: Option<String>,
    }

    let body: ApiErrorBody = resp.json().await.unwrap_or(ApiErrorBody {
        message: None,
        code: None,
        error: None,
    });

    let message = body.message
        .or(body.error)
        .unwrap_or_else(|| format!("HTTP {status}"));
    let code = body.code.unwrap_or_else(|| "unknown".to_owned());

    match status {
        404 => Error::NotFound,
        401 => Error::Unauthorized,
        403 => Error::Forbidden,
        400 => Error::BadRequest(message),
        429 => Error::RateLimited { retry_after: 0 }, // overridden by caller
        _ => Error::Api(ApiError { status_code: status, message, code }),
    }
}

/// Update the in-memory rate-limit state from response headers.
pub(crate) fn update_rate_limit(inner: &ClientInner, headers: &reqwest::header::HeaderMap) {
    let limit = headers
        .get("X-RateLimit-Limit")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());
    let remaining = headers
        .get("X-RateLimit-Remaining")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    if let (Some(limit), Some(remaining)) = (limit, remaining) {
        *inner.rate_limit.lock().unwrap() = Some(RateLimitState { limit, remaining });
    }
}

/// Extract `Retry-After` seconds from response headers.
pub(crate) fn retry_after_secs(headers: &reqwest::header::HeaderMap) -> u64 {
    headers
        .get("Retry-After")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1)
}

/// Execute an HTTP request with retry logic (exponential backoff on 429 / 5xx).
pub(crate) async fn execute_with_retry(
    inner: &ClientInner,
    build_request: impl Fn() -> reqwest::RequestBuilder,
) -> Result<reqwest::Response, Error> {
    let max = inner.config.max_retries;
    let mut attempt = 0u32;

    loop {
        let resp = build_request().send().await?;
        let status = resp.status().as_u16();

        update_rate_limit(inner, resp.headers());

        // Success
        if resp.status().is_success() {
            return Ok(resp);
        }

        // Rate limited
        if status == 429 {
            let retry_after = retry_after_secs(resp.headers());
            if attempt < max {
                attempt += 1;
                tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                continue;
            }
            return Err(Error::RateLimited { retry_after });
        }

        // Server error — retry with backoff
        if status >= 500 && attempt < max {
            attempt += 1;
            let backoff = std::time::Duration::from_millis(200 * (1u64 << attempt));
            tokio::time::sleep(backoff).await;
            continue;
        }

        return Err(parse_api_error(status, resp).await);
    }
}
