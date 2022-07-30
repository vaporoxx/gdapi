use super::error::Result;
use super::{base64, xor};

pub fn decode(input: &str) -> Result<u32> {
	let base64 = base64::decode(input)?;
	let xor = xor::xor(&base64, xor::keys::PASSWORD)?;
	let parsed = xor.parse()?;

	Ok(parsed)
}
