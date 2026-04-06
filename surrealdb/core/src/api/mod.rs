pub mod err;
pub mod invocation;
pub mod middleware;
pub mod path;
pub mod request;
pub mod response;

/// Header name for SurrealDB request ID tracking
pub const X_SURREAL_REQUEST_ID: http::HeaderName =
	surrealdb_core_protocol::api::X_SURREAL_REQUEST_ID;

pub mod format {
	pub use surrealdb_core_protocol::api::format::{
		ANY, CBOR, FLATBUFFERS, JSON, NATIVE, OCTET_STREAM, PLAIN,
	};
}
