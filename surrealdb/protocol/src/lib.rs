//! Shared protocol-layer types and constants.

pub mod api {
	use http::HeaderName;

	/// Header name for SurrealDB request ID tracking
	pub const X_SURREAL_REQUEST_ID: HeaderName =
		HeaderName::from_static("x-surreal-request-id");

	pub mod format {
		//! MIME type string constants for use in HTTP headers

		pub const ANY: &str = "*/*";

		pub const JSON: &str = "application/json";
		pub const CBOR: &str = "application/cbor";
		pub const FLATBUFFERS: &str = "application/vnd.surrealdb.flatbuffers";
		pub const NATIVE: &str = "application/vnd.surrealdb.native";

		pub const PLAIN: &str = "text/plain";
		pub const OCTET_STREAM: &str = "application/octet-stream";
	}
}

pub mod rpc {
	pub mod format {
		pub const PROTOCOLS: [&str; 3] = [
			"json",
			"cbor",
			"flatbuffers",
		];

		#[derive(Debug, Clone, Copy, Eq, PartialEq)]
		pub enum Format {
			Json,
			Cbor,
			Flatbuffers,
			Unsupported,
		}

		impl From<&str> for Format {
			fn from(v: &str) -> Self {
				match v {
					"json" => Format::Json,
					"cbor" => Format::Cbor,
					"flatbuffers" => Format::Flatbuffers,
					_ => Format::Unsupported,
				}
			}
		}
	}

	#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
	#[non_exhaustive]
	pub enum Method {
		Unknown,
		Ping,
		Info,
		Use,
		Signup,
		Signin,
		Authenticate,
		Refresh,
		Invalidate,
		Revoke,
		Reset,
		Kill,
		Live,
		Set,
		Unset,
		Select,
		Insert,
		Create,
		Upsert,
		Update,
		Merge,
		Patch,
		Delete,
		Version,
		Query,
		Relate,
		Run,
		InsertRelation,
		Attach,
		Sessions,
		Detach,
		Begin,
		Commit,
		Cancel,
	}

	impl Method {
		pub fn parse_case_insensitive<S>(s: S) -> Self
		where
			S: AsRef<str>,
		{
			Self::parse(s.as_ref().to_ascii_lowercase().as_str())
		}

		pub fn parse_case_sensitive<S>(s: S) -> Self
		where
			S: AsRef<str>,
		{
			Self::parse(s.as_ref())
		}

		fn parse<S>(s: S) -> Self
		where
			S: AsRef<str>,
		{
			match s.as_ref() {
				"ping" => Self::Ping,
				"info" => Self::Info,
				"use" => Self::Use,
				"signup" => Self::Signup,
				"signin" => Self::Signin,
				"authenticate" => Self::Authenticate,
				"refresh" => Self::Refresh,
				"invalidate" => Self::Invalidate,
				"revoke" => Self::Revoke,
				"reset" => Self::Reset,
				"kill" => Self::Kill,
				"live" => Self::Live,
				"set" | "let" => Self::Set,
				"unset" => Self::Unset,
				"select" => Self::Select,
				"insert" => Self::Insert,
				"create" => Self::Create,
				"upsert" => Self::Upsert,
				"update" => Self::Update,
				"merge" => Self::Merge,
				"patch" => Self::Patch,
				"delete" => Self::Delete,
				"version" => Self::Version,
				"query" => Self::Query,
				"relate" => Self::Relate,
				"run" => Self::Run,
				"insert_relation" => Self::InsertRelation,
				"attach" => Self::Attach,
				"sessions" => Self::Sessions,
				"detach" => Self::Detach,
				"begin" => Self::Begin,
				"commit" => Self::Commit,
				"cancel" => Self::Cancel,
				_ => Self::Unknown,
			}
		}

		pub fn to_str(&self) -> &str {
			match self {
				Self::Unknown => "unknown",
				Self::Ping => "ping",
				Self::Info => "info",
				Self::Use => "use",
				Self::Signup => "signup",
				Self::Signin => "signin",
				Self::Authenticate => "authenticate",
				Self::Refresh => "refresh",
				Self::Invalidate => "invalidate",
				Self::Revoke => "revoke",
				Self::Reset => "reset",
				Self::Kill => "kill",
				Self::Live => "live",
				Self::Set => "set",
				Self::Unset => "unset",
				Self::Select => "select",
				Self::Insert => "insert",
				Self::Create => "create",
				Self::Upsert => "upsert",
				Self::Update => "update",
				Self::Merge => "merge",
				Self::Patch => "patch",
				Self::Delete => "delete",
				Self::Version => "version",
				Self::Query => "query",
				Self::Relate => "relate",
				Self::Run => "run",
				Self::InsertRelation => "insert_relation",
				Self::Attach => "attach",
				Self::Sessions => "sessions",
				Self::Detach => "detach",
				Self::Begin => "begin",
				Self::Commit => "commit",
				Self::Cancel => "cancel",
			}
		}

		pub fn is_valid(&self) -> bool {
			!matches!(self, Self::Unknown)
		}
	}

	impl std::fmt::Display for Method {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.to_str())
		}
	}
}
