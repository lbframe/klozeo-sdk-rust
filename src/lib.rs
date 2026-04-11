//! # klozeo
//!
//! Official Rust SDK for the [Klozeo](https://klozeo.com) Lead Management API.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use klozeo::{Client, LeadInput};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), klozeo::Error> {
//!     let client = Client::new("sk_live_your_api_key");
//!
//!     // Create a lead
//!     let resp = client.leads().create(
//!         LeadInput::builder()
//!             .name("Acme Corporation")
//!             .source("website")
//!             .city("San Francisco")
//!             .email("contact@acme.com")
//!             .rating(4.5)
//!             .tags(vec!["enterprise".into(), "saas".into()])
//!             .build(),
//!     ).await?;
//!
//!     println!("Created: {}", resp.id);
//!     Ok(())
//! }
//! ```
//!
//! ## Filtering
//!
//! ```rust,ignore
//! use klozeo::filters::{city, rating};
//! use klozeo::types::{ListOptions, SortField, SortOrder};
//!
//! let result = client.leads().list(
//!     ListOptions::builder()
//!         .filter(city().eq("Berlin"))
//!         .filter(rating().gte(4.0))
//!         .sort(SortField::Rating, SortOrder::Desc)
//!         .limit(20)
//!         .build(),
//! ).await?;
//! ```
//!
//! ## Streaming
//!
//! ```rust,ignore
//! use futures::StreamExt;
//! use klozeo::filters::city;
//! use klozeo::types::ListOptions;
//!
//! let mut stream = client.leads().stream(
//!     ListOptions::builder().filter(city().eq("Berlin")).build()
//! );
//! while let Some(result) = stream.next().await {
//!     println!("{}", result?.name);
//! }
//! ```

pub mod client;
pub mod errors;
pub mod filters;
pub mod resources;
pub mod types;

// ─── Top-level re-exports ──────────────────────────────────────────────────

pub use client::{Client, ClientConfig};
pub use errors::{ApiError, Error};
pub use types::{
    Attribute,
    AttributeResponse,
    BatchCreateResult,
    BatchCreatedItem,
    BatchErrorItem,
    BatchResult,
    BatchResultItem,
    CreateResponse,
    ExportFormat,
    ExportOptions,
    ExportOptionsBuilder,
    LeadInput,
    LeadInputBuilder,
    LeadResponse,
    ListOptions,
    ListOptionsBuilder,
    ListResult,
    Note,
    RateLimitState,
    ScoreResponse,
    ScoringRule,
    ScoringRuleInput,
    SortField,
    SortOrder,
    UpdateLeadInput,
    UpdateLeadInputBuilder,
    Webhook,
    WebhookInput,
};
