use super::error::Result;
use super::{base64, xor};

pub fn encode(input: &str) -> Result<String> {
	let xor = xor::xor(input, xor::keys::GJP)?;
	let base64 = base64::encode(&xor);

	Ok(base64)
}
