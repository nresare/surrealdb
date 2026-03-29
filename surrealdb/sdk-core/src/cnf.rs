use std::sync::LazyLock;

pub const PROTECTED_PARAM_NAMES: &[&str] = &["access", "auth", "token", "session"];

pub static SURREALDB_USER_AGENT: LazyLock<String> =
	LazyLock::new(|| std::env::var("SURREAL_USER_AGENT").unwrap_or("SurrealDB".to_string()));
