pub mod flatbuffers;
pub mod json;

pub const PROTOCOLS: [&str; 2] = ["json", "flatbuffers"];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Format {
	Json,
	Flatbuffers,
	Unsupported,
}

impl From<&str> for Format {
	fn from(v: &str) -> Self {
		match v {
			"json" => Format::Json,
			"flatbuffers" => Format::Flatbuffers,
			_ => Format::Unsupported,
		}
	}
}
