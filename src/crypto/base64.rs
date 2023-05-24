use base64::prelude::{Engine, BASE64_URL_SAFE};

use super::error::Result;

pub fn decode(input: &str) -> Result<String> {
	let vec = BASE64_URL_SAFE.decode(input)?;
	let string = String::from_utf8(vec)?;

	Ok(string)
}

pub fn encode(input: &str) -> String {
	BASE64_URL_SAFE.encode(input)
}
