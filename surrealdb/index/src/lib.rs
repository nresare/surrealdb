//! Shared index vocabulary extracted from `surrealdb-core`.

pub mod ft {
	/// Reference identifier for a full-text search match.
	pub type MatchRef = u8;
}

pub mod seqdocids {
	/// Internal numeric document identifier used by index structures.
	pub type DocId = u64;
}

pub mod planner {
	use std::fmt::{Display, Formatter};

	#[derive(Clone, Copy, Debug)]
	pub enum RecordStrategy {
		Count,
		KeysOnly,
		KeysAndValues,
	}

	#[derive(Clone, Copy, Debug)]
	pub enum ScanDirection {
		Forward,
		Backward,
	}

	impl Display for ScanDirection {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			match self {
				ScanDirection::Forward => f.write_str("forward"),
				ScanDirection::Backward => f.write_str("backward"),
			}
		}
	}
}
