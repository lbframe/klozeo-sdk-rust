# klozeo

Official Rust SDK for the [Klozeo](https://klozeo.com) Lead Management API.

## Installation

```toml
[dependencies]
klozeo = "0.1"
tokio = { version = "1", features = ["full"] }
```

Use `rustls-tls` instead of the default native TLS:

```toml
klozeo = { version = "0.1", default-features = false, features = ["rustls-tls"] }
```

## Quick Start

```rust
use klozeo::{Client, LeadInput, Attribute};

#[tokio::main]
async fn main() -> Result<(), klozeo::Error> {
    let client = Client::new("sk_live_your_api_key");

    // Create a lead
    let resp = client.leads().create(
        LeadInput::builder()
            .name("Acme Corporation")
            .source("website")
            .city("San Francisco")
            .email("contact@acme.com")
            .rating(4.5)
            .tags(vec!["enterprise".into(), "saas".into()])
            .attributes(vec![
                Attribute::text("industry", "Software"),
                Attribute::number("employees", 500.0),
            ])
            .build(),
    ).await?;

    println!("Created: {}", resp.id);

    // List leads with filters
    use klozeo::filters::{city, rating};
    use klozeo::types::{ListOptions, SortField, SortOrder};

    let result = client.leads().list(
        ListOptions::builder()
            .filter(city().eq("San Francisco"))
            .filter(rating().gte(4.0))
            .sort(SortField::Rating, SortOrder::Desc)
            .limit(20)
            .build(),
    ).await?;

    for lead in &result.leads {
        println!("{} — score: {:.0}", lead.name, lead.score);
    }

    Ok(())
}
```

## Stream (automatic pagination)

```rust
use futures::StreamExt;
use klozeo::filters::city;
use klozeo::types::ListOptions;

let mut stream = client.leads().stream(
    ListOptions::builder()
        .filter(city().eq("Berlin"))
        .build(),
);

while let Some(result) = stream.next().await {
    let lead = result?;
    println!("{}", lead.name);
}
```

## Custom config

```rust
use klozeo::{Client, ClientConfig};
use std::time::Duration;

let client = Client::with_config(
    "sk_live_your_api_key",
    ClientConfig::builder()
        .base_url("https://custom.api.com")
        .timeout(Duration::from_secs(60))
        .max_retries(5)
        .build(),
);
```

## Error handling

```rust
use klozeo::Error;

match client.leads().get("cl_nonexistent").await {
    Ok(lead) => println!("{}", lead.name),
    Err(Error::NotFound) => println!("Lead not found"),
    Err(Error::Unauthorized) => println!("Invalid API key"),
    Err(Error::RateLimited { retry_after }) => {
        println!("Rate limited — retry after {retry_after}s");
    }
    Err(e) => eprintln!("Error: {e}"),
}
```

## Links

- API Reference: <https://docs.klozeo.com>
- crates.io: <https://crates.io/crates/klozeo>
- docs.rs: <https://docs.rs/klozeo>
