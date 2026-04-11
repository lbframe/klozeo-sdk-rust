/// Trait implemented by all filter types. A filter serializes to a single
/// `"logic.operator.field.value"` string that is passed as a repeated
/// `filter=` query parameter.
pub trait Filter: Send + Sync {
    /// Serialize this filter to its `"logic.operator.field.value"` string.
    fn to_param(&self) -> String;
}

// ─── Internal helpers ──────────────────────────────────────────────────────

/// Logic prefix used for combining filters.
#[derive(Debug, Clone, Copy)]
enum Logic {
    And,
    Or,
}

impl Logic {
    fn as_str(self) -> &'static str {
        match self {
            Logic::And => "and",
            Logic::Or => "or",
        }
    }
}

// ─── GenericFilter ─────────────────────────────────────────────────────────

/// A fully-built filter value.
#[derive(Debug, Clone)]
pub struct GenericFilter {
    logic: Logic,
    operator: String,
    field: String,
    value: String,
}

impl Filter for GenericFilter {
    fn to_param(&self) -> String {
        if self.value.is_empty() {
            format!("{}.{}.{}", self.logic.as_str(), self.operator, self.field)
        } else {
            format!("{}.{}.{}.{}", self.logic.as_str(), self.operator, self.field, self.value)
        }
    }
}

// ─── TextFilterBuilder ─────────────────────────────────────────────────────

/// Builder for text-field filters (`name`, `city`, `country`, …).
#[derive(Debug, Clone)]
pub struct TextFilterBuilder {
    logic: Logic,
    field: String,
}

impl TextFilterBuilder {
    fn new(logic: Logic, field: impl Into<String>) -> Self {
        Self { logic, field: field.into() }
    }

    fn make(&self, op: &str, val: impl Into<String>) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: op.to_owned(),
            field: self.field.clone(),
            value: val.into(),
        }
    }

    fn make_no_val(&self, op: &str) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: op.to_owned(),
            field: self.field.clone(),
            value: String::new(),
        }
    }

    /// Equals (case-insensitive).
    pub fn eq(self, value: impl Into<String>) -> GenericFilter {
        self.make("eq", value)
    }
    /// Not equals.
    pub fn neq(self, value: impl Into<String>) -> GenericFilter {
        self.make("neq", value)
    }
    /// Contains substring.
    pub fn contains(self, value: impl Into<String>) -> GenericFilter {
        self.make("contains", value)
    }
    /// Does not contain substring.
    pub fn not_contains(self, value: impl Into<String>) -> GenericFilter {
        self.make("not_contains", value)
    }
    /// Is null or empty string.
    pub fn is_empty(self) -> GenericFilter {
        self.make_no_val("is_empty")
    }
    /// Has a non-empty value.
    pub fn is_not_empty(self) -> GenericFilter {
        self.make_no_val("is_not_empty")
    }
}

// ─── NumberFilterBuilder ───────────────────────────────────────────────────

/// Builder for numeric-field filters (`rating`, `review_count`).
#[derive(Debug, Clone)]
pub struct NumberFilterBuilder {
    logic: Logic,
    field: String,
}

impl NumberFilterBuilder {
    fn new(logic: Logic, field: impl Into<String>) -> Self {
        Self { logic, field: field.into() }
    }

    fn make(&self, op: &str, val: f64) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: op.to_owned(),
            field: self.field.clone(),
            value: val.to_string(),
        }
    }

    /// Equals.
    pub fn eq(self, value: f64) -> GenericFilter { self.make("eq", value) }
    /// Not equals.
    pub fn neq(self, value: f64) -> GenericFilter { self.make("neq", value) }
    /// Greater than.
    pub fn gt(self, value: f64) -> GenericFilter { self.make("gt", value) }
    /// Greater than or equal.
    pub fn gte(self, value: f64) -> GenericFilter { self.make("gte", value) }
    /// Less than.
    pub fn lt(self, value: f64) -> GenericFilter { self.make("lt", value) }
    /// Less than or equal.
    pub fn lte(self, value: f64) -> GenericFilter { self.make("lte", value) }
}

// ─── TagsFilterBuilder ─────────────────────────────────────────────────────

/// Builder for the `tags` array field.
#[derive(Debug, Clone)]
pub struct TagsFilterBuilder {
    logic: Logic,
}

impl TagsFilterBuilder {
    fn new(logic: Logic) -> Self { Self { logic } }

    fn make(&self, op: &str, val: impl Into<String>) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: op.to_owned(),
            field: "tags".to_owned(),
            value: val.into(),
        }
    }

    fn make_no_val(&self, op: &str) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: op.to_owned(),
            field: "tags".to_owned(),
            value: String::new(),
        }
    }

    /// Array contains the given value.
    pub fn contains(self, value: impl Into<String>) -> GenericFilter {
        self.make("array_contains", value)
    }
    /// Array does not contain the given value.
    pub fn not_contains(self, value: impl Into<String>) -> GenericFilter {
        self.make("array_not_contains", value)
    }
    /// Array is empty.
    pub fn is_empty(self) -> GenericFilter { self.make_no_val("array_empty") }
    /// Array has at least one item.
    pub fn is_not_empty(self) -> GenericFilter { self.make_no_val("array_not_empty") }
}

// ─── LocationFilterBuilder ─────────────────────────────────────────────────

/// Builder for the `location` (latitude + longitude) field.
#[derive(Debug, Clone)]
pub struct LocationFilterBuilder {
    logic: Logic,
}

impl LocationFilterBuilder {
    fn new(logic: Logic) -> Self { Self { logic } }

    /// Within a radius of `km` kilometres from the given coordinates.
    pub fn within_radius(self, lat: f64, lng: f64, km: f64) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: "within_radius".to_owned(),
            field: "location".to_owned(),
            value: format!("{lat},{lng},{km}"),
        }
    }
    /// Has coordinates set.
    pub fn is_set(self) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: "is_set".to_owned(),
            field: "location".to_owned(),
            value: String::new(),
        }
    }
    /// Has no coordinates.
    pub fn is_not_set(self) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: "is_not_set".to_owned(),
            field: "location".to_owned(),
            value: String::new(),
        }
    }
}

// ─── AttrFilterBuilder ─────────────────────────────────────────────────────

/// Builder for custom attribute filters (`attr:<name>`).
#[derive(Debug, Clone)]
pub struct AttrFilterBuilder {
    logic: Logic,
    name: String,
}

impl AttrFilterBuilder {
    fn new(logic: Logic, name: impl Into<String>) -> Self {
        Self { logic, name: name.into() }
    }

    fn field(&self) -> String {
        format!("attr:{}", self.name)
    }

    fn make(&self, op: &str, val: impl Into<String>) -> GenericFilter {
        GenericFilter {
            logic: self.logic,
            operator: op.to_owned(),
            field: self.field(),
            value: val.into(),
        }
    }

    /// Text equals (case-insensitive).
    pub fn eq(self, value: impl Into<String>) -> GenericFilter {
        self.make("eq", value)
    }
    /// Text not equals.
    pub fn neq(self, value: impl Into<String>) -> GenericFilter {
        self.make("neq", value)
    }
    /// Text contains substring.
    pub fn contains(self, value: impl Into<String>) -> GenericFilter {
        self.make("contains", value)
    }
    /// Numeric equals.
    pub fn eq_number(self, value: f64) -> GenericFilter {
        self.make("eq", value.to_string())
    }
    /// Numeric greater than.
    pub fn gt(self, value: f64) -> GenericFilter {
        self.make("gt", value.to_string())
    }
    /// Numeric greater than or equal.
    pub fn gte(self, value: f64) -> GenericFilter {
        self.make("gte", value.to_string())
    }
    /// Numeric less than.
    pub fn lt(self, value: f64) -> GenericFilter {
        self.make("lt", value.to_string())
    }
    /// Numeric less than or equal.
    pub fn lte(self, value: f64) -> GenericFilter {
        self.make("lte", value.to_string())
    }
}

// ─── OrBuilder ─────────────────────────────────────────────────────────────

/// Entry point for OR-logic filters.
///
/// ```rust,ignore
/// .filter(or().city().eq("Paris"))
/// ```
#[derive(Debug, Clone)]
pub struct OrBuilder;

impl OrBuilder {
    /// Filter by name (OR logic).
    pub fn name(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "name") }
    /// Filter by city (OR logic).
    pub fn city(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "city") }
    /// Filter by country (OR logic).
    pub fn country(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "country") }
    /// Filter by state (OR logic).
    pub fn state(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "state") }
    /// Filter by category (OR logic).
    pub fn category(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "category") }
    /// Filter by source (OR logic).
    pub fn source(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "source") }
    /// Filter by email (OR logic).
    pub fn email(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "email") }
    /// Filter by phone (OR logic).
    pub fn phone(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "phone") }
    /// Filter by website (OR logic).
    pub fn website(self) -> TextFilterBuilder { TextFilterBuilder::new(Logic::Or, "website") }
    /// Filter by rating (OR logic).
    pub fn rating(self) -> NumberFilterBuilder { NumberFilterBuilder::new(Logic::Or, "rating") }
    /// Filter by review count (OR logic).
    pub fn review_count(self) -> NumberFilterBuilder { NumberFilterBuilder::new(Logic::Or, "review_count") }
    /// Filter by tags (OR logic).
    pub fn tags(self) -> TagsFilterBuilder { TagsFilterBuilder::new(Logic::Or) }
    /// Filter by location (OR logic).
    pub fn location(self) -> LocationFilterBuilder { LocationFilterBuilder::new(Logic::Or) }
    /// Filter by custom attribute (OR logic).
    pub fn attr(self, name: impl Into<String>) -> AttrFilterBuilder {
        AttrFilterBuilder::new(Logic::Or, name)
    }
}

// ─── Top-level constructor functions (AND logic by default) ────────────────

/// Filter leads by name (AND logic).
pub fn name() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "name") }

/// Filter leads by city (AND logic).
///
/// ```rust,ignore
/// city().eq("Berlin")
/// ```
pub fn city() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "city") }

/// Filter leads by country (AND logic).
pub fn country() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "country") }

/// Filter leads by state (AND logic).
pub fn state() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "state") }

/// Filter leads by category (AND logic).
pub fn category() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "category") }

/// Filter leads by source (AND logic).
pub fn source() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "source") }

/// Filter leads by email (AND logic).
pub fn email() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "email") }

/// Filter leads by phone (AND logic).
pub fn phone() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "phone") }

/// Filter leads by website (AND logic).
pub fn website() -> TextFilterBuilder { TextFilterBuilder::new(Logic::And, "website") }

/// Filter leads by rating (AND logic).
///
/// ```rust,ignore
/// rating().gte(4.0)
/// ```
pub fn rating() -> NumberFilterBuilder { NumberFilterBuilder::new(Logic::And, "rating") }

/// Filter leads by review count (AND logic).
pub fn review_count() -> NumberFilterBuilder { NumberFilterBuilder::new(Logic::And, "review_count") }

/// Filter leads by tags (AND logic).
///
/// ```rust,ignore
/// tags().contains("enterprise")
/// ```
pub fn tags() -> TagsFilterBuilder { TagsFilterBuilder::new(Logic::And) }

/// Filter leads by location (AND logic).
///
/// ```rust,ignore
/// location().within_radius(52.52, 13.405, 50.0)
/// ```
pub fn location() -> LocationFilterBuilder { LocationFilterBuilder::new(Logic::And) }

/// Filter leads by a custom attribute (AND logic).
///
/// ```rust,ignore
/// attr("industry").eq("Software")
/// ```
pub fn attr(name: impl Into<String>) -> AttrFilterBuilder {
    AttrFilterBuilder::new(Logic::And, name)
}

/// Begin an OR-logic filter chain.
///
/// ```rust,ignore
/// or().city().eq("Paris")
/// ```
pub fn or() -> OrBuilder { OrBuilder }
