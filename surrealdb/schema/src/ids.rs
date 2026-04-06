use std::fmt::{Display, Formatter};

use revision::{DeserializeRevisioned, Revisioned, SerializeRevisioned, revisioned};
use serde::{Deserialize, Serialize};
use storekey::{BorrowDecode, Encode};

macro_rules! id_type {
	(
		$(#[$meta:meta])*
		$name:ident
	) => {
		$(#[$meta])*
		#[derive(
			Debug,
			Clone,
			Copy,
			PartialEq,
			Eq,
			PartialOrd,
			Ord,
			Hash,
			Serialize,
			Deserialize,
			Encode,
			BorrowDecode,
		)]
		#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
		#[repr(transparent)]
		pub struct $name(pub u32);

		impl Revisioned for $name {
			fn revision() -> u16 {
				1
			}
		}

		impl SerializeRevisioned for $name {
			#[inline]
			fn serialize_revisioned<W: std::io::Write>(
				&self,
				writer: &mut W,
			) -> Result<(), revision::Error> {
				SerializeRevisioned::serialize_revisioned(&self.0, writer)
			}
		}

		impl DeserializeRevisioned for $name {
			#[inline]
			fn deserialize_revisioned<R: std::io::Read>(
				reader: &mut R,
			) -> Result<Self, revision::Error> {
				DeserializeRevisioned::deserialize_revisioned(reader).map($name)
			}
		}

		impl Display for $name {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}

		impl From<u32> for $name {
			fn from(value: u32) -> Self {
				Self(value)
			}
		}
	};
}

id_type!(
	/// A namespace identifier stored in the catalog.
	NamespaceId
);

id_type!(
	/// A database identifier stored in the catalog.
	DatabaseId
);

id_type!(
	/// A table identifier stored in the catalog.
	TableId
);

id_type!(
	/// An index identifier stored in the catalog.
	IndexId
);

#[revisioned(revision = 1)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[repr(transparent)]
pub struct BucketId(pub u32);

impl Display for BucketId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<u32> for BucketId {
	fn from(value: u32) -> Self {
		Self(value)
	}
}
