use super::error::Result;

pub mod keys {
	pub const GJP: &str = "37526";
}

pub fn xor(input: &str, key: &str) -> Result<String> {
	let vec = input.bytes().zip(key.bytes().cycle()).map(|(a, b)| a ^ b).collect();
	let string = String::from_utf8(vec)?;

	Ok(string)
}
