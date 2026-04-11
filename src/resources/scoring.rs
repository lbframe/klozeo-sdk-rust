use std::sync::Arc;

use serde::Deserialize;

use crate::client::{execute_with_retry, ClientInner};
use crate::errors::Error;
use crate::types::{ScoreResponse, ScoringRule, ScoringRuleInput};

/// Access scoring-rule API endpoints.
///
/// Obtain an instance via [`crate::Client::scoring`].
#[derive(Clone)]
pub struct ScoringResource {
    inner: Arc<ClientInner>,
}

impl ScoringResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    fn rules_url(&self) -> String {
        format!("{}/scoring-rules", self.inner.config.base_url)
    }

    fn rule_url(&self, id: &str) -> String {
        format!("{}/scoring-rules/{}", self.inner.config.base_url, id)
    }

    /// List all scoring rules for the account.
    ///
    /// ```rust,ignore
    /// let rules = client.scoring().list_rules().await?;
    /// ```
    pub async fn list_rules(&self) -> Result<Vec<ScoringRule>, Error> {
        let url = self.rules_url();
        let resp = execute_with_retry(&self.inner, || self.inner.http.get(&url)).await?;

        #[derive(Deserialize)]
        struct RulesResponse {
            rules: Vec<ScoringRule>,
        }
        let body = resp.json::<RulesResponse>().await?;
        Ok(body.rules)
    }

    /// Create a scoring rule.
    ///
    /// ```rust,ignore
    /// let rule = client.scoring().create_rule(ScoringRuleInput {
    ///     name: "Contact completeness".into(),
    ///     expression: "(email != nil ? 30 : 0) + (phone != nil ? 30 : 0)".into(),
    ///     priority: Some(1),
    /// }).await?;
    /// ```
    pub async fn create_rule(&self, input: ScoringRuleInput) -> Result<ScoringRule, Error> {
        let url = self.rules_url();
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.post(&url).json(&input)
        })
        .await?;
        Ok(resp.json::<ScoringRule>().await?)
    }

    /// Get a single scoring rule by ID.
    ///
    /// ```rust,ignore
    /// let rule = client.scoring().get_rule("rule-uuid").await?;
    /// ```
    pub async fn get_rule(&self, id: &str) -> Result<ScoringRule, Error> {
        let url = self.rule_url(id);
        let resp = execute_with_retry(&self.inner, || self.inner.http.get(&url)).await?;
        Ok(resp.json::<ScoringRule>().await?)
    }

    /// Update a scoring rule.
    ///
    /// ```rust,ignore
    /// client.scoring().update_rule("rule-uuid", ScoringRuleInput {
    ///     name: "Updated rule".into(),
    ///     expression: "rating * 20".into(),
    ///     priority: Some(2),
    /// }).await?;
    /// ```
    pub async fn update_rule(&self, id: &str, input: ScoringRuleInput) -> Result<(), Error> {
        let url = self.rule_url(id);
        execute_with_retry(&self.inner, || {
            self.inner.http.put(&url).json(&input)
        })
        .await?;
        Ok(())
    }

    /// Delete a scoring rule by ID.
    ///
    /// ```rust,ignore
    /// client.scoring().delete_rule("rule-uuid").await?;
    /// ```
    pub async fn delete_rule(&self, id: &str) -> Result<(), Error> {
        let url = self.rule_url(id);
        execute_with_retry(&self.inner, || self.inner.http.delete(&url)).await?;
        Ok(())
    }

    /// Recalculate and persist the score for a single lead.
    ///
    /// Returns the newly computed score value.
    ///
    /// ```rust,ignore
    /// let score = client.scoring().recalculate("cl_01234567-...").await?;
    /// println!("New score: {score}");
    /// ```
    pub async fn recalculate(&self, lead_id: &str) -> Result<f64, Error> {
        let url = format!("{}/leads/{}/score", self.inner.config.base_url, lead_id);
        let resp = execute_with_retry(&self.inner, || self.inner.http.post(&url)).await?;
        let body = resp.json::<ScoreResponse>().await?;
        Ok(body.score)
    }
}
