use surrealdb_types::Error as TypesError;

use crate::cnf::PROTECTED_PARAM_NAMES;

pub mod format {
	pub mod flatbuffers {
		pub use surrealdb_types::{decode, encode};
	}
}

mod response;

pub use response::{DbResponse, DbResult, DbResultStats};

pub const SESSION_ID: &str = "session";

pub fn check_protected_param(key: &str) -> Result<(), TypesError> {
	if PROTECTED_PARAM_NAMES.contains(&key) {
		return Err(TypesError::validation(
			format!("Cannot set protected variable: {key}"),
			surrealdb_types::ValidationError::Parse,
		));
	}
	Ok(())
}
