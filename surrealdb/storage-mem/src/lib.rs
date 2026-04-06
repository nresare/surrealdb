//! Shared metadata for the memory storage backend.

pub const NAME: &str = "memory";
pub const ALIAS: &str = "mem";

pub fn matches_scheme(scheme: &str) -> bool {
	scheme == NAME || scheme == ALIAS
}
