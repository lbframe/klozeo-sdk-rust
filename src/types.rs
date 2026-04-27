use serde::{Deserialize, Serialize};

// ─── Lead ──────────────────────────────────────────────────────────────────

/// A lead as returned by the Klozeo API.
#[derive(Debug, Clone, Deserialize)]
pub struct LeadResponse {
    /// Unique lead identifier (e.g. `"cl_01234567-..."`).
    pub id: String,
    /// Computed lead score (0–100).
    pub score: f64,
    /// Lead name.
    pub name: String,
    /// Traffic source (e.g. `"website"`, `"import"`).
    pub source: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub rating: Option<f64>,
    pub review_count: Option<u32>,
    pub category: Option<String>,
    /// Array of string tags attached to this lead.
    pub tags: Vec<String>,
    pub source_id: Option<String>,
    pub logo_url: Option<String>,
    pub status: Option<String>,
    /// Custom dynamic attributes.
    pub attributes: Vec<AttributeResponse>,
    /// Unix timestamp (seconds) when the lead was created.
    pub created_at: i64,
    /// Unix timestamp (seconds) of the last structural update.
    pub updated_at: i64,
    /// Unix timestamp (seconds) of the last inbound push or merge.
    pub last_interaction_at: i64,
}

// ─── Attribute ─────────────────────────────────────────────────────────────

/// A custom dynamic attribute attached to a lead (as returned by the API).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeResponse {
    /// UUID of this attribute record.
    pub id: String,
    /// Attribute name.
    pub name: String,
    /// Attribute type tag (`"text"`, `"number"`, `"bool"`, `"list"`, `"object"`).
    #[serde(rename = "type")]
    pub attr_type: String,
    /// Raw JSON value.
    pub value: serde_json::Value,
}

/// A custom attribute to attach when creating or updating a lead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute name.
    pub name: String,
    /// Attribute type tag (`"text"`, `"number"`, `"bool"`, `"list"`, `"object"`).
    #[serde(rename = "type")]
    pub attr_type: String,
    /// JSON-serialisable value.
    pub value: serde_json::Value,
}

impl Attribute {
    /// Create a text attribute.
    ///
    /// ```rust,ignore
    /// Attribute::text("industry", "Software")
    /// ```
    pub fn text(name: &str, value: &str) -> Self {
        Self {
            name: name.to_owned(),
            attr_type: "text".to_owned(),
            value: serde_json::Value::String(value.to_owned()),
        }
    }

    /// Create a number attribute.
    ///
    /// ```rust,ignore
    /// Attribute::number("employees", 500.0)
    /// ```
    pub fn number(name: &str, value: f64) -> Self {
        Self {
            name: name.to_owned(),
            attr_type: "number".to_owned(),
            value: serde_json::json!(value),
        }
    }

    /// Create a boolean attribute.
    ///
    /// ```rust,ignore
    /// Attribute::bool("verified", true)
    /// ```
    pub fn bool(name: &str, value: bool) -> Self {
        Self {
            name: name.to_owned(),
            attr_type: "bool".to_owned(),
            value: serde_json::Value::Bool(value),
        }
    }

    /// Create a list attribute.
    ///
    /// ```rust,ignore
    /// Attribute::list("products", vec!["CRM".into(), "ERP".into()])
    /// ```
    pub fn list(name: &str, value: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            attr_type: "list".to_owned(),
            value: serde_json::to_value(value).expect("Vec<String> is always serialisable"),
        }
    }

    /// Create an object attribute from any JSON value.
    ///
    /// ```rust,ignore
    /// Attribute::object("social", serde_json::json!({"linkedin": "https://linkedin.com/company/acme"}))
    /// ```
    pub fn object(name: &str, value: serde_json::Value) -> Self {
        Self {
            name: name.to_owned(),
            attr_type: "object".to_owned(),
            value,
        }
    }
}

// ─── LeadInput ─────────────────────────────────────────────────────────────

/// Input for creating a new lead.
///
/// Use [`LeadInput::builder`] to construct this type.
#[derive(Debug, Clone, Serialize)]
pub struct LeadInput {
    /// Lead name (required).
    pub name: String,
    /// Traffic source (required).
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
}

impl LeadInput {
    /// Returns a builder for `LeadInput`.
    pub fn builder() -> LeadInputBuilder {
        LeadInputBuilder::default()
    }
}

/// Builder for [`LeadInput`].
#[derive(Debug, Default)]
pub struct LeadInputBuilder {
    name: Option<String>,
    source: Option<String>,
    description: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    postal_code: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    phone: Option<String>,
    email: Option<String>,
    website: Option<String>,
    rating: Option<f64>,
    review_count: Option<u32>,
    category: Option<String>,
    tags: Vec<String>,
    source_id: Option<String>,
    logo_url: Option<String>,
    status: Option<String>,
    attributes: Vec<Attribute>,
}

impl LeadInputBuilder {
    /// Set the lead name (required).
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Set the traffic source (required).
    pub fn source(mut self, v: impl Into<String>) -> Self {
        self.source = Some(v.into());
        self
    }
    /// Set the description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Set the street address.
    pub fn address(mut self, v: impl Into<String>) -> Self {
        self.address = Some(v.into());
        self
    }
    /// Set the city.
    pub fn city(mut self, v: impl Into<String>) -> Self {
        self.city = Some(v.into());
        self
    }
    /// Set the state / province.
    pub fn state(mut self, v: impl Into<String>) -> Self {
        self.state = Some(v.into());
        self
    }
    /// Set the country.
    pub fn country(mut self, v: impl Into<String>) -> Self {
        self.country = Some(v.into());
        self
    }
    /// Set the postal code.
    pub fn postal_code(mut self, v: impl Into<String>) -> Self {
        self.postal_code = Some(v.into());
        self
    }
    /// Set the latitude.
    pub fn latitude(mut self, v: f64) -> Self {
        self.latitude = Some(v);
        self
    }
    /// Set the longitude.
    pub fn longitude(mut self, v: f64) -> Self {
        self.longitude = Some(v);
        self
    }
    /// Set the phone number.
    pub fn phone(mut self, v: impl Into<String>) -> Self {
        self.phone = Some(v.into());
        self
    }
    /// Set the email address.
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.email = Some(v.into());
        self
    }
    /// Set the website URL.
    pub fn website(mut self, v: impl Into<String>) -> Self {
        self.website = Some(v.into());
        self
    }
    /// Set the rating (0–5).
    pub fn rating(mut self, v: f64) -> Self {
        self.rating = Some(v);
        self
    }
    /// Set the review count.
    pub fn review_count(mut self, v: u32) -> Self {
        self.review_count = Some(v);
        self
    }
    /// Set the category.
    pub fn category(mut self, v: impl Into<String>) -> Self {
        self.category = Some(v.into());
        self
    }
    /// Set the tags array.
    pub fn tags(mut self, v: Vec<String>) -> Self {
        self.tags = v;
        self
    }
    /// Set the external source ID.
    pub fn source_id(mut self, v: impl Into<String>) -> Self {
        self.source_id = Some(v.into());
        self
    }
    /// Set the logo URL.
    pub fn logo_url(mut self, v: impl Into<String>) -> Self {
        self.logo_url = Some(v.into());
        self
    }
    /// Set the pipeline status.
    pub fn status(mut self, v: impl Into<String>) -> Self {
        self.status = Some(v.into());
        self
    }
    /// Set custom attributes.
    pub fn attributes(mut self, v: Vec<Attribute>) -> Self {
        self.attributes = v;
        self
    }
    /// Build the [`LeadInput`].
    ///
    /// # Panics
    /// Panics if `name` or `source` have not been set.
    pub fn build(self) -> LeadInput {
        LeadInput {
            name: self.name.expect("LeadInput requires `name`"),
            source: self.source.expect("LeadInput requires `source`"),
            description: self.description,
            address: self.address,
            city: self.city,
            state: self.state,
            country: self.country,
            postal_code: self.postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            phone: self.phone,
            email: self.email,
            website: self.website,
            rating: self.rating,
            review_count: self.review_count,
            category: self.category,
            tags: self.tags,
            source_id: self.source_id,
            logo_url: self.logo_url,
            status: self.status,
            attributes: self.attributes,
        }
    }
}

// ─── UpdateLeadInput ───────────────────────────────────────────────────────

/// Input for partially updating a lead. All fields are optional.
///
/// Use [`UpdateLeadInput::builder`] to construct this type.
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateLeadInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl UpdateLeadInput {
    /// Returns a builder for `UpdateLeadInput`.
    pub fn builder() -> UpdateLeadInputBuilder {
        UpdateLeadInputBuilder::default()
    }
}

/// Builder for [`UpdateLeadInput`].
#[derive(Debug, Default)]
pub struct UpdateLeadInputBuilder(UpdateLeadInput);

impl UpdateLeadInputBuilder {
    /// Set the lead name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.0.name = Some(v.into());
        self
    }
    /// Set the traffic source.
    pub fn source(mut self, v: impl Into<String>) -> Self {
        self.0.source = Some(v.into());
        self
    }
    /// Set the description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.0.description = Some(v.into());
        self
    }
    /// Set the street address.
    pub fn address(mut self, v: impl Into<String>) -> Self {
        self.0.address = Some(v.into());
        self
    }
    /// Set the city.
    pub fn city(mut self, v: impl Into<String>) -> Self {
        self.0.city = Some(v.into());
        self
    }
    /// Set the state / province.
    pub fn state(mut self, v: impl Into<String>) -> Self {
        self.0.state = Some(v.into());
        self
    }
    /// Set the country.
    pub fn country(mut self, v: impl Into<String>) -> Self {
        self.0.country = Some(v.into());
        self
    }
    /// Set the postal code.
    pub fn postal_code(mut self, v: impl Into<String>) -> Self {
        self.0.postal_code = Some(v.into());
        self
    }
    /// Set the latitude.
    pub fn latitude(mut self, v: f64) -> Self {
        self.0.latitude = Some(v);
        self
    }
    /// Set the longitude.
    pub fn longitude(mut self, v: f64) -> Self {
        self.0.longitude = Some(v);
        self
    }
    /// Set the phone number.
    pub fn phone(mut self, v: impl Into<String>) -> Self {
        self.0.phone = Some(v.into());
        self
    }
    /// Set the email address.
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.0.email = Some(v.into());
        self
    }
    /// Set the website URL.
    pub fn website(mut self, v: impl Into<String>) -> Self {
        self.0.website = Some(v.into());
        self
    }
    /// Set the rating.
    pub fn rating(mut self, v: f64) -> Self {
        self.0.rating = Some(v);
        self
    }
    /// Set the review count.
    pub fn review_count(mut self, v: u32) -> Self {
        self.0.review_count = Some(v);
        self
    }
    /// Set the category.
    pub fn category(mut self, v: impl Into<String>) -> Self {
        self.0.category = Some(v.into());
        self
    }
    /// Set the tags array.
    pub fn tags(mut self, v: Vec<String>) -> Self {
        self.0.tags = Some(v);
        self
    }
    /// Set the external source ID.
    pub fn source_id(mut self, v: impl Into<String>) -> Self {
        self.0.source_id = Some(v.into());
        self
    }
    /// Set the logo URL.
    pub fn logo_url(mut self, v: impl Into<String>) -> Self {
        self.0.logo_url = Some(v.into());
        self
    }
    /// Set the pipeline status.
    pub fn status(mut self, v: impl Into<String>) -> Self {
        self.0.status = Some(v.into());
        self
    }
    /// Build the [`UpdateLeadInput`].
    pub fn build(self) -> UpdateLeadInput {
        self.0
    }
}

// ─── CreateResponse ────────────────────────────────────────────────────────

/// Response body for a successful lead creation request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateResponse {
    /// ID of the created (or merged) lead.
    pub id: String,
    /// Human-readable status message.
    pub message: String,
    /// Unix timestamp (seconds) when the lead was created.
    pub created_at: i64,
    /// `true` when an existing lead was merged instead of a new one created.
    pub duplicate: Option<bool>,
    /// ID of a similar lead when a low-confidence potential duplicate was found.
    pub potential_duplicate_id: Option<String>,
}

// ─── ListResult ────────────────────────────────────────────────────────────

/// Paginated list of leads.
#[derive(Debug, Clone, Deserialize)]
pub struct ListResult {
    /// Leads in this page.
    pub leads: Vec<LeadResponse>,
    /// Opaque cursor for the next page. Pass to [`ListOptions`] as `cursor`.
    pub next_cursor: Option<String>,
    /// Whether more pages are available.
    pub has_more: bool,
    /// Number of leads in this page.
    pub count: u32,
}

// ─── BatchResult ───────────────────────────────────────────────────────────

/// A single successfully created item in a batch create response.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreatedItem {
    /// Zero-based index of this item in the original request array.
    pub index: usize,
    /// ID of the created lead.
    pub id: String,
    /// Unix timestamp (seconds) of creation.
    pub created_at: i64,
}

/// A single error item in a batch create response.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchErrorItem {
    /// Zero-based index of the failing item.
    pub index: usize,
    /// Human-readable error message.
    pub message: String,
}

/// Response for [`LeadsResource::batch_create`].
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateResult {
    /// Successfully created leads.
    pub created: Vec<BatchCreatedItem>,
    /// Leads that failed to create.
    pub errors: Vec<BatchErrorItem>,
    /// Total number of items submitted.
    pub total: u32,
    /// Number of successfully created items.
    pub success: u32,
    /// Number of items that failed.
    pub failed: u32,
}

/// A single result row in a batch update or delete response.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchResultItem {
    /// Zero-based index of this item.
    pub index: usize,
    /// Lead ID.
    pub id: String,
    /// Whether the operation succeeded for this item.
    pub success: bool,
    /// Optional error message when `success` is `false`.
    pub message: Option<String>,
}

/// Response for [`LeadsResource::batch_update`] and [`LeadsResource::batch_delete`].
#[derive(Debug, Clone, Deserialize)]
pub struct BatchResult {
    /// Per-item results.
    pub results: Vec<BatchResultItem>,
    /// Total number of items submitted.
    pub total: u32,
    /// Number of successful operations.
    pub success: u32,
    /// Number of failed operations.
    pub failed: u32,
}

// ─── Note ──────────────────────────────────────────────────────────────────

/// A note attached to a lead.
#[derive(Debug, Clone, Deserialize)]
pub struct Note {
    /// Note ID (e.g. `"note_01234567-..."`).
    pub id: String,
    /// ID of the parent lead.
    pub lead_id: String,
    /// Text content of the note.
    pub content: String,
    /// Unix timestamp (seconds) of creation.
    pub created_at: i64,
    /// Unix timestamp (seconds) of last update.
    pub updated_at: i64,
}

// ─── ScoringRule ───────────────────────────────────────────────────────────

/// A scoring rule that contributes to lead scores.
#[derive(Debug, Clone, Deserialize)]
pub struct ScoringRule {
    /// UUID of the rule.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Expression string (CEL-like).
    pub expression: String,
    /// Evaluation priority — lower value means higher priority.
    pub priority: i32,
    /// Unix timestamp (seconds) of creation.
    pub created_at: i64,
    /// Unix timestamp (seconds) of last update.
    pub updated_at: i64,
}

/// Input for creating or updating a scoring rule.
#[derive(Debug, Clone, Serialize)]
pub struct ScoringRuleInput {
    /// Human-readable name.
    pub name: String,
    /// Expression string (CEL-like).
    pub expression: String,
    /// Evaluation priority (optional, defaults to 0 on the API).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

// ─── Webhook ───────────────────────────────────────────────────────────────

/// A webhook registered for your account.
#[derive(Debug, Clone, Deserialize)]
pub struct Webhook {
    /// UUID of the webhook.
    pub id: String,
    /// Endpoint URL that receives POST notifications.
    pub url: String,
    /// Events this webhook is subscribed to.
    pub events: Option<Vec<String>>,
    /// Whether the webhook is currently active.
    pub active: bool,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
}

/// Input for creating a webhook.
#[derive(Debug, Clone, Serialize)]
pub struct WebhookInput {
    /// Endpoint URL (required).
    pub url: String,
    /// Events to subscribe to (optional — defaults to all events).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Payload signing secret (optional, never returned by the API).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

// ─── Score response ────────────────────────────────────────────────────────

/// Response body for the recalculate-score endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ScoreResponse {
    /// Lead ID.
    pub id: String,
    /// Newly computed score.
    pub score: f64,
}

// ─── Export ────────────────────────────────────────────────────────────────

/// File format for the export endpoint.
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    /// Comma-separated values.
    Csv,
    /// JSON array.
    Json,
    /// Excel workbook.
    Xlsx,
}

impl ExportFormat {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ExportFormat::Csv => "csv",
            ExportFormat::Json => "json",
            ExportFormat::Xlsx => "xlsx",
        }
    }
}

// ─── ExportOptions ─────────────────────────────────────────────────────────

/// Options for the export endpoint (filters + sort).
#[derive(Default)]
pub struct ExportOptions {
    /// Filters to apply.
    pub filters: Vec<Box<dyn crate::filters::Filter>>,
    /// Field to sort by.
    pub sort_by: Option<SortField>,
    /// Sort direction.
    pub sort_order: Option<SortOrder>,
}

impl ExportOptions {
    /// Returns a builder for `ExportOptions`.
    pub fn builder() -> ExportOptionsBuilder {
        ExportOptionsBuilder::default()
    }
}

/// Builder for [`ExportOptions`].
#[derive(Default)]
pub struct ExportOptionsBuilder {
    filters: Vec<Box<dyn crate::filters::Filter>>,
    sort_by: Option<SortField>,
    sort_order: Option<SortOrder>,
}

impl ExportOptionsBuilder {
    /// Add a filter.
    pub fn filter(mut self, f: impl crate::filters::Filter + 'static) -> Self {
        self.filters.push(Box::new(f));
        self
    }
    /// Set sort field and order together.
    pub fn sort(mut self, field: SortField, order: SortOrder) -> Self {
        self.sort_by = Some(field);
        self.sort_order = Some(order);
        self
    }
    /// Build the [`ExportOptions`].
    pub fn build(self) -> ExportOptions {
        ExportOptions {
            filters: self.filters,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
        }
    }
}

// ─── ListOptions ───────────────────────────────────────────────────────────

/// Options for the list-leads endpoint (filters, sort, pagination).
#[derive(Default)]
pub struct ListOptions {
    /// Filters to apply (combined with AND/OR logic per filter).
    pub filters: Vec<Box<dyn crate::filters::Filter>>,
    /// Field to sort by.
    pub sort_by: Option<SortField>,
    /// Sort direction.
    pub sort_order: Option<SortOrder>,
    /// Maximum number of results per page (API default: 50, max: 1 000).
    pub limit: Option<u32>,
    /// Opaque pagination cursor from a previous [`ListResult`].
    pub cursor: Option<String>,
}

impl ListOptions {
    /// Returns a builder for `ListOptions`.
    pub fn builder() -> ListOptionsBuilder {
        ListOptionsBuilder::default()
    }
}

/// Builder for [`ListOptions`].
#[derive(Default)]
pub struct ListOptionsBuilder {
    filters: Vec<Box<dyn crate::filters::Filter>>,
    sort_by: Option<SortField>,
    sort_order: Option<SortOrder>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl ListOptionsBuilder {
    /// Add a filter.
    pub fn filter(mut self, f: impl crate::filters::Filter + 'static) -> Self {
        self.filters.push(Box::new(f));
        self
    }
    /// Set sort field and order together.
    pub fn sort(mut self, field: SortField, order: SortOrder) -> Self {
        self.sort_by = Some(field);
        self.sort_order = Some(order);
        self
    }
    /// Set the page size limit.
    pub fn limit(mut self, v: u32) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set the pagination cursor.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Build the [`ListOptions`].
    pub fn build(self) -> ListOptions {
        ListOptions {
            filters: self.filters,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
            limit: self.limit,
            cursor: self.cursor,
        }
    }
}

// ─── SortField / SortOrder ─────────────────────────────────────────────────

/// Fields available for sorting the leads list.
#[derive(Debug, Clone)]
pub enum SortField {
    Name,
    City,
    Country,
    State,
    Category,
    Source,
    Email,
    Phone,
    Website,
    Rating,
    ReviewCount,
    CreatedAt,
    UpdatedAt,
    LastInteractionAt,
    /// Sort by a custom attribute: `SortField::Attr("employees".into())`.
    Attr(String),
}

impl SortField {
    /// Convert to the query-parameter string expected by the API.
    pub fn as_str(&self) -> std::borrow::Cow<'_, str> {
        match self {
            SortField::Name => "name".into(),
            SortField::City => "city".into(),
            SortField::Country => "country".into(),
            SortField::State => "state".into(),
            SortField::Category => "category".into(),
            SortField::Source => "source".into(),
            SortField::Email => "email".into(),
            SortField::Phone => "phone".into(),
            SortField::Website => "website".into(),
            SortField::Rating => "rating".into(),
            SortField::ReviewCount => "review_count".into(),
            SortField::CreatedAt => "created_at".into(),
            SortField::UpdatedAt => "updated_at".into(),
            SortField::LastInteractionAt => "last_interaction_at".into(),
            SortField::Attr(name) => format!("attr:{name}").into(),
        }
    }
}

/// Sort direction.
#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    /// Ascending order (A → Z, oldest first).
    Asc,
    /// Descending order (Z → A, newest first).
    Desc,
}

impl SortOrder {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

// ─── RateLimitState ────────────────────────────────────────────────────────

/// The most recently observed rate-limit headers from the API.
#[derive(Debug, Clone)]
pub struct RateLimitState {
    /// Maximum requests allowed per window.
    pub limit: u64,
    /// Requests remaining in the current window.
    pub remaining: u64,
}
