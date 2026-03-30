use super::Level;

#[derive(Clone, Default, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Auth {
	level: Level,
}

impl Auth {
	pub fn new(level: Level) -> Self {
		Self {
			level,
		}
	}

	pub fn level(&self) -> &Level {
		&self.level
	}

	pub fn is_anon(&self) -> bool {
		matches!(self.level, Level::No)
	}

	pub fn is_record(&self) -> bool {
		matches!(self.level, Level::Record(_, _, _))
	}
}
