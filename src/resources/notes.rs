use std::sync::Arc;

use serde::Deserialize;
use serde_json::json;

use crate::client::{execute_with_retry, ClientInner};
use crate::errors::Error;
use crate::types::Note;

/// Access note-related API endpoints.
///
/// Obtain an instance via [`crate::Client::notes`].
#[derive(Clone)]
pub struct NotesResource {
    inner: Arc<ClientInner>,
}

impl NotesResource {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    /// Create a note on a lead.
    ///
    /// ```rust,ignore
    /// let note = client.notes().create("cl_01234567-...", "Initial contact made").await?;
    /// ```
    pub async fn create(&self, lead_id: &str, content: &str) -> Result<Note, Error> {
        let url = format!(
            "{}/leads/{}/notes",
            self.inner.config.base_url, lead_id
        );
        let body = json!({ "content": content });
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.post(&url).json(&body)
        })
        .await?;
        Ok(resp.json::<Note>().await?)
    }

    /// List all notes for a lead.
    ///
    /// ```rust,ignore
    /// let notes = client.notes().list("cl_01234567-...").await?;
    /// ```
    pub async fn list(&self, lead_id: &str) -> Result<Vec<Note>, Error> {
        let url = format!(
            "{}/leads/{}/notes",
            self.inner.config.base_url, lead_id
        );
        let resp = execute_with_retry(&self.inner, || self.inner.http.get(&url)).await?;

        #[derive(Deserialize)]
        struct NotesResponse {
            notes: Vec<Note>,
        }
        let body = resp.json::<NotesResponse>().await?;
        Ok(body.notes)
    }

    /// Update the content of a note.
    ///
    /// ```rust,ignore
    /// let note = client.notes().update("note_01234567-...", "Updated content").await?;
    /// ```
    pub async fn update(&self, note_id: &str, content: &str) -> Result<Note, Error> {
        let url = format!("{}/notes/{}", self.inner.config.base_url, note_id);
        let body = json!({ "content": content });
        let resp = execute_with_retry(&self.inner, || {
            self.inner.http.put(&url).json(&body)
        })
        .await?;
        Ok(resp.json::<Note>().await?)
    }

    /// Delete a note by ID. Returns `Ok(())` on success (HTTP 204).
    ///
    /// ```rust,ignore
    /// client.notes().delete("note_01234567-...").await?;
    /// ```
    pub async fn delete(&self, note_id: &str) -> Result<(), Error> {
        let url = format!("{}/notes/{}", self.inner.config.base_url, note_id);
        execute_with_retry(&self.inner, || self.inner.http.delete(&url)).await?;
        Ok(())
    }
}
