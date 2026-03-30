use crate::types::PublicValue;

pub fn encode(value: PublicValue) -> anyhow::Result<Vec<u8>> {
	encode_str(value).map(|x| x.into_bytes())
}

pub fn encode_str(value: PublicValue) -> anyhow::Result<String> {
	let v = value.into_json_value();
	Ok(serde_json::to_string(&v).expect("serialization to json string should not fail"))
}
