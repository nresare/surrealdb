use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

use jsonwebtoken::{Algorithm, Header};
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

pub static HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::HS512));

#[derive(Clone, Eq, PartialEq, PartialOrd, SurrealValue, Hash)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Audience {
	Single(String),
	Multiple(Vec<String>),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Claims {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub iat: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub nbf: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exp: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub iss: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sub: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub aud: Option<Audience>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jti: Option<String>,
	#[serde(alias = "ns")]
	#[serde(alias = "NS")]
	#[serde(rename = "NS")]
	#[serde(alias = "https://surrealdb.com/ns")]
	#[serde(alias = "https://surrealdb.com/namespace")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ns: Option<String>,
	#[serde(alias = "db")]
	#[serde(alias = "DB")]
	#[serde(rename = "DB")]
	#[serde(alias = "https://surrealdb.com/db")]
	#[serde(alias = "https://surrealdb.com/database")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub db: Option<String>,
	#[serde(alias = "ac")]
	#[serde(alias = "AC")]
	#[serde(rename = "AC")]
	#[serde(alias = "https://surrealdb.com/ac")]
	#[serde(alias = "https://surrealdb.com/access")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ac: Option<String>,
	#[serde(alias = "id")]
	#[serde(alias = "ID")]
	#[serde(rename = "ID")]
	#[serde(alias = "https://surrealdb.com/id")]
	#[serde(alias = "https://surrealdb.com/record")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(alias = "rl")]
	#[serde(alias = "RL")]
	#[serde(rename = "RL")]
	#[serde(alias = "https://surrealdb.com/rl")]
	#[serde(alias = "https://surrealdb.com/roles")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roles: Option<Vec<String>>,
	#[serde(flatten)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_claims: Option<HashMap<String, serde_json::Value>>,
}
