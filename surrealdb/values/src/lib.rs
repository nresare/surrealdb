//! Extracted low-level value types.

pub mod bytes;
pub mod file;

pub use bytes::Bytes;
pub use file::File;

/// Marker type for a different serialization format for values which does not
/// encode type information that is not required for indexing.
pub enum IndexFormat {}
