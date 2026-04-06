//! Schema-layer catalog types being split out of `surrealdb-core`.

mod api;
mod ids;

pub use api::ApiMethod;
pub use ids::{BucketId, DatabaseId, IndexId, NamespaceId, TableId};
