use serde::Serialize;

/// Pagination is applied for some domains of Opsgenie Rest API,
/// meaning that only a certain number of resources will be returned within each response.
///
/// This structure represents the pagination query parameters used in the API.
///
/// [Corresponding API page](https://docs.opsgenie.com/docs/pagination)
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct Pagination {
    /// Starting point for the result set
    pub offset: u32,
    /// Maximum number of items to provide in the result.
    /// At time of writing, Opsgenie requires this value to be <= 100.
    pub limit: u32,
    /// Name of the field that result set will be sorted by.
    /// See the related section for each request.
    pub sort: Option<String>,
    /// Page direction to apply for the given offset
    pub direction: Option<Direction>,
    /// Sorting order of the result set
    pub order: Option<Order>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: Self::DEFAULT_LIMIT,
            sort: None,
            direction: None,
            order: None,
        }
    }
}

impl Pagination {
    /// Limit value used by default, if not set explicitly.
    pub const DEFAULT_LIMIT: u32 = 20;

    /// Creates a new `Pagination` object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `offset` field.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    /// Sets the `direction` field.
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Sets the `limit` field.
    ///
    /// Does not check if `limit` value will be accepted by the
    /// Opsgenie API.
    /// If interested in a safe maximum limit, use [`Pagination::with_max_limit`].
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Sets the limit to [`PAGINATION_MAX_LIMIT`](crate::limits::PAGINATION_MAX_LIMIT).
    pub fn with_max_limit(mut self) -> Self {
        self.limit = crate::limits::PAGINATION_MAX_LIMIT;
        self
    }

    /// Sets the `sort` field.
    pub fn with_sort(mut self, sort: impl Into<Option<String>>) -> Self {
        self.sort = sort.into();
        self
    }

    /// Sets the `order` field.
    pub fn with_order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    /// Creates an object that represents the next page.
    pub fn next(mut self) -> Self {
        self.offset += self.limit;
        self
    }

    /// Creates an object that represents the previous page.
    /// Returns `None` if `offset` is less than `limit`.
    pub fn prev(mut self) -> Option<Self> {
        if self.offset < self.limit {
            return None;
        }
        self.offset -= self.limit;
        Some(self)
    }
}

/// Page direction to apply for the given offset
#[derive(Debug, Default, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Greater than the given offset (Default Value)
    #[default]
    Next,
    /// Less than the given offset
    Prev,
}

/// Sorting order of the result set
#[derive(Debug, Default, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    /// Sort result set in descending order (Default Value)
    #[default]
    Desc,
    /// Sort result set in ascending order
    Asc,
}
