use std::sync::LazyLock;

pub const PROTECTED_PARAM_NAMES: &[&str] = &["access", "auth", "token", "session"];

pub static SURREALDB_USER_AGENT: LazyLock<String> =
	LazyLock::new(|| format!("SurrealDB-RustSDK/{}", env!("CARGO_PKG_VERSION")));
