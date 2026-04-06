pub use surrealdb_query_runtime::access_mode::{AccessMode, CombineAccessModes};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_combine_read_only() {
		assert_eq!(AccessMode::ReadOnly.combine(AccessMode::ReadOnly), AccessMode::ReadOnly);
	}

	#[test]
	fn test_combine_read_write() {
		assert_eq!(AccessMode::ReadOnly.combine(AccessMode::ReadWrite), AccessMode::ReadWrite);
		assert_eq!(AccessMode::ReadWrite.combine(AccessMode::ReadOnly), AccessMode::ReadWrite);
		assert_eq!(AccessMode::ReadWrite.combine(AccessMode::ReadWrite), AccessMode::ReadWrite);
	}

	#[test]
	fn test_default() {
		assert_eq!(AccessMode::default(), AccessMode::ReadOnly);
	}

	#[test]
	fn test_is_read_only() {
		assert!(AccessMode::ReadOnly.is_read_only());
		assert!(!AccessMode::ReadWrite.is_read_only());
	}

	#[test]
	fn test_is_read_write() {
		assert!(!AccessMode::ReadOnly.is_read_write());
		assert!(AccessMode::ReadWrite.is_read_write());
	}

	#[test]
	fn test_combine_all() {
		let modes = vec![AccessMode::ReadOnly, AccessMode::ReadOnly];
		assert_eq!(modes.into_iter().combine_all(), AccessMode::ReadOnly);

		let modes = vec![AccessMode::ReadOnly, AccessMode::ReadWrite, AccessMode::ReadOnly];
		assert_eq!(modes.into_iter().combine_all(), AccessMode::ReadWrite);

		let modes: Vec<AccessMode> = vec![];
		assert_eq!(modes.into_iter().combine_all(), AccessMode::ReadOnly);
	}
}
