//! Shared database orchestration vocabulary extracted from `surrealdb-core`.

pub mod session {
	use std::fmt;
	use std::str::FromStr;

	#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
	pub enum NewPlannerStrategy {
		#[default]
		BestEffortReadOnlyStatements,
		ComputeOnly,
		AllReadOnlyStatements,
	}

	impl fmt::Display for NewPlannerStrategy {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match self {
				Self::BestEffortReadOnlyStatements => f.write_str("best-effort"),
				Self::ComputeOnly => f.write_str("compute-only"),
				Self::AllReadOnlyStatements => f.write_str("all-read-only"),
			}
		}
	}

	impl FromStr for NewPlannerStrategy {
		type Err = String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"best-effort" => Ok(Self::BestEffortReadOnlyStatements),
				"compute-only" => Ok(Self::ComputeOnly),
				"all-read-only" => Ok(Self::AllReadOnlyStatements),
				_ => Err(format!(
					"unknown planner strategy: '{s}' (expected 'best-effort', 'compute-only', or 'all-read-only')"
				)),
			}
		}
	}
}

pub mod context {
	use std::sync::Arc;
	use std::sync::atomic::{AtomicBool, Ordering};

	#[derive(Default, Clone)]
	pub struct Canceller {
		cancelled: Arc<AtomicBool>,
	}

	impl Canceller {
		pub fn new(cancelled: Arc<AtomicBool>) -> Canceller {
			Canceller {
				cancelled,
			}
		}

		pub fn cancel(&self) {
			self.cancelled.store(true, Ordering::Relaxed);
		}
	}

	#[cfg(feature = "scripting")]
	#[derive(Clone, Debug, Default)]
	pub struct Cancellation {
		deadline: Option<web_time::Instant>,
		cancellations: Vec<Arc<AtomicBool>>,
	}

	#[cfg(feature = "scripting")]
	impl Cancellation {
		pub fn new(
			deadline: Option<web_time::Instant>,
			cancellations: Vec<Arc<AtomicBool>>,
		) -> Cancellation {
			Self {
				deadline,
				cancellations,
			}
		}

		pub fn is_done(&self) -> bool {
			self.deadline.map(|d| d <= web_time::Instant::now()).unwrap_or(false)
				|| self.cancellations.iter().any(|c| c.load(Ordering::Relaxed))
		}
	}
}
