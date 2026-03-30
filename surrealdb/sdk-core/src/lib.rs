#![allow(clippy::mutable_key_type)]

#[macro_use]
extern crate tracing;

pub mod api;
pub mod cnf;
pub mod dbs;
pub mod iam;
pub mod kvs;
pub mod rpc;

pub(crate) mod types {
	pub use surrealdb_types::{
		Action as PublicAction, Array as PublicArray, Kind as PublicKind,
		Notification as PublicNotification, Number as PublicNumber, Object as PublicObject,
		SurrealValue, Uuid as PublicUuid, Value as PublicValue,
	};
}
