//! The module defining the key value store.
//! Everything related the transaction for the key value store is defined in the `tx.rs` file.
//! This module enables the following operations on the key value store:
//! - get
//! - set
//! - delete
//! - put
//!
//! These operations can be processed by the following storage engines:
//! - `indxdb`: WASM based database to store data in the browser
//! - `rocksdb`: [RocksDB](https://github.com/facebook/rocksdb) an embeddable persistent key-value
//!   store for fast storage
//! - `tikv`: [TiKV](https://github.com/tikv/tikv) a distributed, and transactional key-value
//!   database
//! - `mem`: in-memory database

pub mod config;
pub mod export;

mod api;
mod batch;
mod clock;
mod ds;
mod err;
mod into;
mod key;
mod scanner;
mod threadpool;
mod timestamp;
mod tr;
mod tx;
mod util;

mod indxdb;
mod mem;
mod rocksdb;
mod surrealkv;
mod tikv;

#[cfg(test)]
mod tests;

pub(crate) mod cache;
pub(crate) mod index;
pub(crate) mod sequences;
pub(crate) mod slowlog;
pub(crate) mod tasklease;
pub(crate) mod version;

pub use api::{ScanLimit, Transactable};
pub(crate) use ds::TransactionFactory;
pub use ds::requirements::{TransactionBuilderFactoryRequirements, TransactionBuilderRequirements};
pub use ds::{
	Datastore, DatastoreFlavor, Metric, Metrics, TransactionBuilder, TransactionBuilderFactory,
};
pub use err::{Error, Result};
pub(crate) use key::{KVKey, KVValue, impl_kv_key_storekey, impl_kv_value_revisioned};
pub use scanner::{Direction, Scanner};
pub use surrealdb_storage::{IntoBytes, Key, LockType, TransactionType, Val, Version};
pub use timestamp::{
	BoxTimeStamp, BoxTimeStampImpl, HlcTimeStamp, HlcTimeStampImpl, IncTimeStampImpl,
	MAX_TIMESTAMP_BYTES, TimeStamp, TimeStampImpl,
};
pub use tr::Transactor;
pub(crate) use tx::CachePolicy;
pub use tx::Transaction;
