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
