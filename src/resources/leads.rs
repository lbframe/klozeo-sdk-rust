use std::sync::Arc;

use bytes::Bytes;
use futures::{stream, Stream, StreamExt};
use serde::Deserialize;
use serde_json::json;

use crate::client::{execute_with_retry, ClientInner};
use crate::errors::Error;
use crate::filters::Filter;
use crate::types::{
    BatchCreateResult, BatchResult, CreateResponse, ExportFormat, ExportOptions, LeadInput,
    LeadResponse, ListOptions, ListResult, UpdateLeadInput,
};

/// Access lead-related API endpoints.
///
/// Obtain an instance via [`crate::Client::leads`].
#[derive(Clone)]
pub struct LeadsResource {
    inner: Arc<ClientInner>,
}

impl LeadsResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/leads{}", self.inner.config.base_url, path)
    }

    /// Build query pairs from a [`ListOptions`] reference (without consuming it).
    fn build_list_params(
        filters: &[Box<dyn Filter>],
        sort_by: Option<&crate::types::SortField>,
        sort_order: Option<crate::types::SortOrder>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = Vec::new();
        for f in filters {
            params.push(("filter".to_owned(), f.to_param()));
        }
        if let Some(field) = sort_by {
            params.push(("sort_by".to_owned(), field.as_str().into_owned()));
        }
        if let Some(order) = sort_order {
            params.push(("sort_order".to_owned(), order.as_str().to_owned()));
        }
        if let Some(l) = limit {
            params.push(("limit".to_owned(), l.to_string()));
        }
        if let Some(c) = cursor {
            params.push(("cursor".to_owned(), c.to_owned()));
        }
        params
    }

    /// Create a new lead. Returns [`CreateResponse`] which includes the new
    /// lead's ID, creation timestamp, and deduplication hints.
    ///
    /// ```rust,ignore
    /// let resp = client.leads().create(
    ///     LeadInput::builder().name("Acme").source("website").build()
    /// ).await?;
    /// ```
    pub async fn create(&self, input: LeadInput) -> Result<CreateResponse, Error> {
        let url = self.url("");
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.post(&url).json(&input)
        })
        .await?;
        Ok(resp.json::<CreateResponse>().await?)
    }

    /// Retrieve a single lead by ID.
    ///
    /// ```rust,ignore
    /// let lead = client.leads().get("cl_01234567-...").await?;
    /// ```
    pub async fn get(&self, id: &str) -> Result<LeadResponse, Error> {
        let url = self.url(&format!("/{id}"));
        let resp = execute_with_retry(&self.inner, || self.inner.http.get(&url)).await?;
        Ok(resp.json::<LeadResponse>().await?)
    }

    /// Partially update a lead. Only fields set on `input` are changed.
    ///
    /// ```rust,ignore
    /// let lead = client.leads().update(
    ///     "cl_01234567-...",
    ///     UpdateLeadInput::builder().rating(4.8).build(),
    /// ).await?;
    /// ```
    pub async fn update(&self, id: &str, input: UpdateLeadInput) -> Result<LeadResponse, Error> {
        let url = self.url(&format!("/{id}"));
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.put(&url).json(&input)
        })
        .await?;
        Ok(resp.json::<LeadResponse>().await?)
    }

    /// Delete a lead by ID. Returns `Ok(())` on success (HTTP 204).
    ///
    /// ```rust,ignore
    /// client.leads().delete("cl_01234567-...").await?;
    /// ```
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        let url = self.url(&format!("/{id}"));
        execute_with_retry(&self.inner, || self.inner.http.delete(&url)).await?;
        Ok(())
    }

    /// List leads with optional filters, sorting, and pagination.
    ///
    /// ```rust,ignore
    /// use klozeo::filters::{city, rating};
    /// use klozeo::types::{ListOptions, SortField, SortOrder};
    ///
    /// let result = client.leads().list(
    ///     ListOptions::builder()
    ///         .filter(city().eq("Berlin"))
    ///         .filter(rating().gte(4.0))
    ///         .sort(SortField::Rating, SortOrder::Desc)
    ///         .limit(20)
    ///         .build()
    /// ).await?;
    /// ```
    pub async fn list(&self, opts: ListOptions) -> Result<ListResult, Error> {
        let url = self.url("");
        let params = Self::build_list_params(
            &opts.filters,
            opts.sort_by.as_ref(),
            opts.sort_order,
            opts.limit,
            opts.cursor.as_deref(),
        );
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.get(&url).query(&params)
        })
        .await?;
        Ok(resp.json::<ListResult>().await?)
    }

    /// Return a [`Stream`] that transparently pages through all leads matching
    /// `opts`, yielding one [`LeadResponse`] at a time.
    ///
    /// ```rust,ignore
    /// use futures::StreamExt;
    /// use klozeo::filters::city;
    /// use klozeo::types::ListOptions;
    ///
    /// let mut stream = client.leads().stream(
    ///     ListOptions::builder().filter(city().eq("Berlin")).build()
    /// );
    /// while let Some(lead) = stream.next().await {
    ///     println!("{}", lead?.name);
    /// }
    /// ```
    pub fn stream(
        &self,
        opts: ListOptions,
    ) -> impl Stream<Item = Result<LeadResponse, Error>> + '_ {
        // Split opts into its reusable parts (filters can't be cloned easily,
        // so we capture the whole opts and rebuild params from them each page).
        let resource = self.clone();

        stream::unfold(
            (opts, false),
            move |(opts, done)| {
                let resource = resource.clone();
                async move {
                    if done {
                        return None;
                    }
                    let url = resource.url("");
                    let params = Self::build_list_params(
                        &opts.filters,
                        opts.sort_by.as_ref(),
                        opts.sort_order,
                        opts.limit,
                        opts.cursor.as_deref(),
                    );
                    let resp = execute_with_retry(&resource.inner, || {
                        resource.inner.http.get(&url).query(&params)
                    })
                    .await;
                    let page = match resp {
                        Ok(r) => match r.json::<ListResult>().await {
                            Ok(p) => p,
                            Err(e) => return Some((stream::iter(vec![Err(Error::Network(e))]), (opts, true))),
                        },
                        Err(e) => return Some((stream::iter(vec![Err(e)]), (opts, true))),
                    };

                    let has_more = page.has_more;
                    let next_cursor = page.next_cursor.clone();
                    let items: Vec<Result<LeadResponse, Error>> =
                        page.leads.into_iter().map(Ok).collect();

                    let next_opts = ListOptions {
                        filters: opts.filters,
                        sort_by: opts.sort_by,
                        sort_order: opts.sort_order,
                        limit: opts.limit,
                        cursor: next_cursor,
                    };

                    Some((stream::iter(items), (next_opts, !has_more)))
                }
            },
        )
        .flatten()
    }

    /// Create up to 100 leads in a single request.
    ///
    /// ```rust,ignore
    /// let result = client.leads().batch_create(vec![
    ///     LeadInput::builder().name("Lead 1").source("import").build(),
    ///     LeadInput::builder().name("Lead 2").source("import").build(),
    /// ]).await?;
    /// println!("Created: {}, Failed: {}", result.success, result.failed);
    /// ```
    pub async fn batch_create(&self, leads: Vec<LeadInput>) -> Result<BatchCreateResult, Error> {
        let url = self.url("/batch");
        let body = json!({ "leads": leads });
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.post(&url).json(&body)
        })
        .await?;
        Ok(resp.json::<BatchCreateResult>().await?)
    }

    /// Update up to 100 leads with the same data in a single request.
    ///
    /// ```rust,ignore
    /// let result = client.leads().batch_update(
    ///     vec!["cl_aaa...".into(), "cl_bbb...".into()],
    ///     UpdateLeadInput::builder().category("Technology").build(),
    /// ).await?;
    /// ```
    pub async fn batch_update(
        &self,
        ids: Vec<String>,
        data: UpdateLeadInput,
    ) -> Result<BatchResult, Error> {
        let url = self.url("/batch");
        let body = json!({ "ids": ids, "data": data });
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.put(&url).json(&body)
        })
        .await?;
        Ok(resp.json::<BatchResult>().await?)
    }

    /// Delete up to 100 leads in a single request.
    ///
    /// ```rust,ignore
    /// let result = client.leads().batch_delete(vec!["cl_aaa...".into()]).await?;
    /// ```
    pub async fn batch_delete(&self, ids: Vec<String>) -> Result<BatchResult, Error> {
        let url = self.url("/batch");
        let body = json!({ "ids": ids });
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.delete(&url).json(&body)
        })
        .await?;
        Ok(resp.json::<BatchResult>().await?)
    }

    /// Export all leads matching the given options as raw bytes.
    ///
    /// ```rust,ignore
    /// use klozeo::types::ExportFormat;
    ///
    /// let bytes = client.leads().export(ExportFormat::Csv, None).await?;
    /// tokio::fs::write("leads.csv", bytes).await?;
    /// ```
    pub async fn export(
        &self,
        format: ExportFormat,
        opts: Option<ExportOptions>,
    ) -> Result<Bytes, Error> {
        let url = format!("{}/leads/export", self.inner.config.base_url);
        let mut params: Vec<(String, String)> = vec![
            ("format".to_owned(), format.as_str().to_owned()),
        ];
        if let Some(ref o) = opts {
            for f in &o.filters {
                params.push(("filter".to_owned(), f.to_param()));
            }
            if let Some(ref field) = o.sort_by {
                params.push(("sort_by".to_owned(), field.as_str().into_owned()));
            }
            if let Some(order) = o.sort_order {
                params.push(("sort_order".to_owned(), order.as_str().to_owned()));
            }
        }
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.get(&url).query(&params)
        })
        .await?;
        Ok(resp.bytes().await?)
    }
}

/// Internal list response (used by stream — same as ListResult).
#[derive(Deserialize)]
struct _ListResponse {
    leads: Vec<LeadResponse>,
    next_cursor: Option<String>,
    has_more: bool,
}
