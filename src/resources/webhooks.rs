use std::sync::Arc;

use serde::Deserialize;

use crate::client::{execute_with_retry, ClientInner};
use crate::errors::Error;
use crate::types::{Webhook, WebhookInput};

/// Access webhook API endpoints.
///
/// Obtain an instance via [`crate::Client::webhooks`].
#[derive(Clone)]
pub struct WebhooksResource {
    inner: Arc<ClientInner>,
}

impl WebhooksResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    fn url(&self) -> String {
        format!("{}/webhooks", self.inner.config.base_url)
    }

    fn url_with_id(&self, id: &str) -> String {
        format!("{}/webhooks/{}", self.inner.config.base_url, id)
    }

    /// List all webhooks registered for the account.
    ///
    /// ```rust,ignore
    /// let webhooks = client.webhooks().list().await?;
    /// ```
    pub async fn list(&self) -> Result<Vec<Webhook>, Error> {
        let url = self.url();
        let resp = execute_with_retry(&self.inner, || self.inner.http.get(&url)).await?;

        #[derive(Deserialize)]
        struct WebhooksResponse {
            webhooks: Vec<Webhook>,
        }
        let body = resp.json::<WebhooksResponse>().await?;
        Ok(body.webhooks)
    }

    /// Register a new webhook.
    ///
    /// ```rust,ignore
    /// use klozeo::types::WebhookInput;
    ///
    /// let webhook = client.webhooks().create(WebhookInput {
    ///     url: "https://example.com/hooks/klozeo".into(),
    ///     events: Some(vec!["lead.created".into(), "lead.updated".into()]),
    ///     secret: Some("your-signing-secret".into()),
    /// }).await?;
    /// ```
    pub async fn create(&self, input: WebhookInput) -> Result<Webhook, Error> {
        let url = self.url();
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.post(&url).json(&input)
        })
        .await?;
        Ok(resp.json::<Webhook>().await?)
    }

    /// Delete a webhook by ID.
    ///
    /// ```rust,ignore
    /// client.webhooks().delete("webhook-uuid").await?;
    /// ```
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let url = self.url_with_id(id);
        execute_with_retry(&self.inner, || self.inner.http.delete(&url)).await?;
        Ok(())
    }
}
