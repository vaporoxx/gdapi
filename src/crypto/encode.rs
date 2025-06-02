use base64::prelude::*;
use sha1::{Digest, Sha1};

pub fn base64(input: &str) -> String {
	BASE64_URL_SAFE.encode(input)
}

pub fn gjp2(input: &str) -> String {
	let mut hash = Sha1::new();

	hash.update(input.as_bytes());
	hash.update(b"mI29fmAnxgTs");

	format!("{:x}", hash.finalize())
}
