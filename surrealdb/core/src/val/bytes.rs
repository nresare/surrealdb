pub use surrealdb_values::Bytes;

#[cfg(test)]
mod tests {
	use crate::val::{Bytes, Value};

	#[test]
	fn serialize() {
		let val = Value::Bytes(Bytes::from(vec![1, 2, 3, 5]));
		let serialized: Vec<u8> = revision::to_vec(&val).unwrap();
		let deserialized: Value = revision::from_slice(&serialized).unwrap();
		assert_eq!(val, deserialized);
	}
}
