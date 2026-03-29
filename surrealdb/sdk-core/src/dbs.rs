use std::fmt;
use std::time::Duration;

use revision::revisioned;
use serde::{Deserialize, Serialize};
use surrealdb_types::{Error as TypesError, ErrorDetails, Kind, SurrealValue, Value, kind, object};
use web_time::Instant;

#[revisioned(revision = 1)]
#[derive(
	Debug,
	Copy,
	Clone,
	Default,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
	Serialize,
	Deserialize,
	SurrealValue,
)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged, lowercase)]
#[serde(rename_all = "lowercase")]
pub enum QueryType {
	#[default]
	#[surreal(value = none)]
	Other,
	Live,
	Kill,
}

impl fmt::Display for QueryType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			QueryType::Other => "other".fmt(f),
			QueryType::Live => "live".fmt(f),
			QueryType::Kill => "kill".fmt(f),
		}
	}
}

#[derive(Debug, Clone)]
pub struct QueryResult {
	pub time: Duration,
	pub result: Result<Value, TypesError>,
	pub query_type: QueryType,
}

impl QueryResult {
	pub fn output(self) -> Result<Value, TypesError> {
		self.result
	}
}

fn into_query_result_value(error: &TypesError) -> Value {
	let mut details = error.details().clone().into_value();

	if let Value::Object(ref mut obj) = details {
		obj.insert("result", error.message().to_string());
		details
	} else {
		Value::Object(object! {
			result: "Failed to serialise error",
			kind: "Internal",
		})
	}
}

fn from_query_result_value(value: Value) -> Result<TypesError, TypesError> {
	let Value::Object(mut map) = value else {
		return Err(TypesError::internal("Expected object for query result error".to_string()));
	};
	let message = map
		.remove("result")
		.ok_or_else(|| {
			TypesError::internal("Missing result (message) for query result error".to_string())
		})?
		.into_string()
		.map_err(|e| TypesError::internal(e.to_string()))?;
	let details = ErrorDetails::from_value(Value::Object(map)).unwrap_or(ErrorDetails::Internal);
	Ok(TypesError::from_details(message, details))
}

impl SurrealValue for QueryResult {
	fn kind_of() -> Kind {
		kind!(
			{
				status: "OK",
				time: string,
				result: any,
				query_type: (QueryType::kind_of()),
			} | {
				status: "ERR",
				time: string,
				result: string,
				kind: string,
				details: any,
				query_type: (QueryType::kind_of()),
			}
		)
	}

	fn is_value(value: &Value) -> bool {
		value.is_object_and(|map| {
			map.get("status").is_some_and(Status::is_value)
				&& map.get("time").is_some_and(Value::is_string)
				&& map.get("result").is_some()
				&& map.get("type").is_some_and(QueryType::is_value)
		})
	}

	fn into_value(self) -> Value {
		let mut map = object! {
			status: Status::from(&self.result).into_value(),
			time: format!("{:?}", self.time).into_value(),
			type: self.query_type.into_value(),
		};
		match self.result {
			Ok(v) => {
				map.insert("result", v);
			}
			Err(e) => {
				let err_val = into_query_result_value(&e);
				if let Value::Object(err_obj) = err_val {
					for (k, v) in err_obj.into_inner() {
						map.insert(k, v);
					}
				}
			}
		}
		Value::Object(map)
	}

	fn from_value(value: Value) -> Result<Self, TypesError> {
		let Value::Object(mut map) = value else {
			return Err(TypesError::internal("Expected object for QueryResult".to_string()));
		};
		let Some(status) = map.remove("status") else {
			return Err(TypesError::internal("Expected status for QueryResult".to_string()));
		};
		let Some(time) = map.remove("time") else {
			return Err(TypesError::internal("Expected time for QueryResult".to_string()));
		};
		let Some(result) = map.remove("result") else {
			return Err(TypesError::internal("Expected result for QueryResult".to_string()));
		};

		let status = Status::from_value(status)?;
		let query_type =
			map.remove("type").map(QueryType::from_value).transpose()?.unwrap_or_default();

		let time = humantime::parse_duration(
			&time.into_string().map_err(|e| TypesError::internal(e.to_string()))?,
		)
		.map_err(|e| TypesError::internal(e.to_string()))?;

		let result = match status {
			Status::Ok => Ok(Value::from_value(result)?),
			Status::Err => {
				map.insert("result".to_string(), result);
				Err(from_query_result_value(Value::Object(map))?)
			}
		};

		Ok(QueryResult {
			time,
			result,
			query_type,
		})
	}
}

pub struct QueryResultBuilder {
	start_time: Instant,
	result: Result<Value, TypesError>,
	query_type: QueryType,
}

impl QueryResultBuilder {
	pub fn started_now() -> Self {
		Self {
			start_time: Instant::now(),
			result: Ok(Value::None),
			query_type: QueryType::Other,
		}
	}

	pub fn instant_none() -> QueryResult {
		QueryResult {
			time: Duration::ZERO,
			result: Ok(Value::None),
			query_type: QueryType::Other,
		}
	}

	pub fn with_result(mut self, result: Result<Value, TypesError>) -> Self {
		self.result = result;
		self
	}

	pub fn with_query_type(mut self, query_type: QueryType) -> Self {
		self.query_type = query_type;
		self
	}

	pub fn finish(self) -> QueryResult {
		QueryResult {
			time: self.start_time.elapsed(),
			result: self.result,
			query_type: self.query_type,
		}
	}

	pub fn finish_with_result(self, result: Result<Value, TypesError>) -> QueryResult {
		QueryResult {
			time: self.start_time.elapsed(),
			result,
			query_type: self.query_type,
		}
	}
}

#[revisioned(revision = 1)]
#[derive(Debug, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[serde(rename_all = "UPPERCASE")]
#[surreal(untagged, uppercase)]
pub enum Status {
	Ok,
	Err,
}

impl Status {
	pub fn is_ok(&self) -> bool {
		matches!(self, Status::Ok)
	}

	pub fn is_err(&self) -> bool {
		matches!(self, Status::Err)
	}
}

impl<'a, T, E> From<&'a Result<T, E>> for Status {
	fn from(result: &'a Result<T, E>) -> Self {
		match result {
			Ok(_) => Status::Ok,
			Err(_) => Status::Err,
		}
	}
}

impl Serialize for QueryResult {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.clone().into_value().serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for QueryResult {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let value = Value::deserialize(deserializer)?;
		QueryResult::from_value(value).map_err(serde::de::Error::custom)
	}
}
