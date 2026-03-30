mod method;
mod request;
mod response;

pub mod format;

pub use format::Format;
pub use method::Method;
pub use request::Request;
pub use response::{DbResponse, DbResult, DbResultStats};

use crate::cnf::PROTECTED_PARAM_NAMES;

pub fn invalid_request() -> surrealdb_types::Error {
	surrealdb_types::Error::validation("Invalid request".to_string(), None)
}

pub fn check_protected_param(key: &str) -> Result<(), surrealdb_types::Error> {
	if PROTECTED_PARAM_NAMES.contains(&key) {
		return Err(surrealdb_types::Error::validation(
			format!("Cannot set protected variable: {key}"),
			None,
		));
	}
	Ok(())
}
