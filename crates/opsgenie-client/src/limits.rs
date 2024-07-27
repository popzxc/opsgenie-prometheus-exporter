//! This module contains limits currently shown by the Opsgenie API
//! documentation. Since these limits can change at any time, they are
//! not enforced by the SDK, but still provided as constants that user
//! may choose to use.

/// Maximum value for the `limit` field in pagination queries.
pub const PAGINATION_MAX_LIMIT: u32 = 100;
