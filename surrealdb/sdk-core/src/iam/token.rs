use std::fmt;

use surrealdb_types::SurrealValue;

#[derive(Clone, Eq, PartialEq, PartialOrd, SurrealValue, Hash)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Token {
	Access(String),
	WithRefresh {
		access: String,
		refresh: String,
	},
}

impl fmt::Debug for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Token::Access(_) => write!(f, "Token::Access(REDACTED)"),
			Token::WithRefresh {
				..
			} => write!(f, "Token::WithRefresh {{ access: REDACTED, refresh: REDACTED }}"),
		}
	}
}
