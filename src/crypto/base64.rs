use super::error::Result;

pub fn decode(input: &str) -> Result<String> {
	let vec = base64::decode_config(input, base64::URL_SAFE)?;
	let string = String::from_utf8(vec)?;

	Ok(string)
}

pub fn encode(input: &str) -> String {
	base64::encode_config(input, base64::URL_SAFE)
}
