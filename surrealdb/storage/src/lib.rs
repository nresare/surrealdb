//! Shared storage vocabulary extracted from `surrealdb-core`.

use std::borrow::Cow;

use bytes::Bytes;

/// The key part of a key-value pair. An alias for [`Vec<u8>`].
pub type Key = Vec<u8>;

/// The value part of a key-value pair. An alias for [`Vec<u8>`].
pub type Val = Vec<u8>;

/// The Version part of a key-value pair. An alias for [`u64`].
pub type Version = u64;

/// Specifies whether the transaction is read-only or writeable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TransactionType {
	Read,
	Write,
}

/// Specifies whether the transaction is optimistic or pessimistic.
#[derive(Copy, Clone)]
pub enum LockType {
	Pessimistic,
	Optimistic,
}

impl From<bool> for LockType {
	fn from(value: bool) -> Self {
		match value {
			true => LockType::Pessimistic,
			false => LockType::Optimistic,
		}
	}
}

/// An optimised trait for converting values to bytes only when needed.
pub trait IntoBytes {
	/// Convert the key to a slice of bytes.
	fn as_slice(&self) -> &[u8];
	/// Convert the key to a vector of bytes.
	fn into_vec(self) -> Vec<u8>;
	/// Convert the key to an owned bytes slice.
	fn into_bytes(self) -> Bytes;
}

impl IntoBytes for &[u8] {
	fn as_slice(&self) -> &[u8] {
		self
	}

	fn into_vec(self) -> Vec<u8> {
		self.to_vec()
	}

	fn into_bytes(self) -> Bytes {
		Bytes::copy_from_slice(self)
	}
}

impl IntoBytes for Vec<u8> {
	fn as_slice(&self) -> &[u8] {
		self.as_slice()
	}

	fn into_vec(self) -> Vec<u8> {
		self
	}

	fn into_bytes(self) -> Bytes {
		Bytes::from(self)
	}
}

impl IntoBytes for &Vec<u8> {
	fn as_slice(&self) -> &[u8] {
		&self[..]
	}

	fn into_vec(self) -> Vec<u8> {
		(*self).clone()
	}

	fn into_bytes(self) -> Bytes {
		Bytes::copy_from_slice(&self[..])
	}
}

impl IntoBytes for Bytes {
	fn as_slice(&self) -> &[u8] {
		self.as_ref()
	}

	fn into_vec(self) -> Vec<u8> {
		self.to_vec()
	}

	fn into_bytes(self) -> Bytes {
		self
	}
}

impl IntoBytes for &Bytes {
	fn as_slice(&self) -> &[u8] {
		self.as_ref()
	}

	fn into_vec(self) -> Vec<u8> {
		self.to_vec()
	}

	fn into_bytes(self) -> Bytes {
		self.clone()
	}
}

impl IntoBytes for &str {
	fn as_slice(&self) -> &[u8] {
		self.as_bytes()
	}

	fn into_vec(self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}

	fn into_bytes(self) -> Bytes {
		Bytes::copy_from_slice(self.as_bytes())
	}
}

impl IntoBytes for String {
	fn as_slice(&self) -> &[u8] {
		self.as_bytes()
	}

	fn into_vec(self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}

	fn into_bytes(self) -> Bytes {
		Bytes::from(self.into_bytes())
	}
}

impl IntoBytes for &String {
	fn as_slice(&self) -> &[u8] {
		self.as_bytes()
	}

	fn into_vec(self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}

	fn into_bytes(self) -> Bytes {
		Bytes::copy_from_slice(self.as_bytes())
	}
}

impl IntoBytes for Box<[u8]> {
	fn as_slice(&self) -> &[u8] {
		self.as_ref()
	}

	fn into_vec(self) -> Vec<u8> {
		self.as_ref().to_vec()
	}

	fn into_bytes(self) -> Bytes {
		Bytes::from(self)
	}
}

impl<'a> IntoBytes for Cow<'a, [u8]> {
	fn as_slice(&self) -> &[u8] {
		self.as_ref()
	}

	fn into_vec(self) -> Vec<u8> {
		match self {
			Cow::Borrowed(s) => s.to_vec(),
			Cow::Owned(v) => v,
		}
	}

	fn into_bytes(self) -> Bytes {
		match self {
			Cow::Borrowed(s) => Bytes::copy_from_slice(s),
			Cow::Owned(v) => Bytes::from(v),
		}
	}
}
