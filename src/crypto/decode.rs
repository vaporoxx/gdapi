use base64::prelude::{BASE64_URL_SAFE, Engine};

use crate::error::Result;

pub fn base64(input: &str) -> Result<String> {
	let vec = BASE64_URL_SAFE.decode(input)?;
	let string = String::from_utf8(vec)?;

	Ok(string)
}

pub fn password(input: &str) -> Result<u32> {
	let base64 = BASE64_URL_SAFE.decode(input)?;
	let xor = xor(&base64, b"26364")?;
	let parsed = xor.parse()?;

	Ok(parsed)
}

pub fn xor(input: &[u8], key: &[u8]) -> Result<String> {
	let vec = input.iter().zip(key.iter().cycle()).map(|(a, b)| a ^ b).collect();
	let string = String::from_utf8(vec)?;

	Ok(string)
}
