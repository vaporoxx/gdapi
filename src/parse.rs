use crate::data::{Gauntlet, Level, LoginUser, MapPack, User};
use gdapi_crypto::decode;
use std::collections::HashMap;

fn parse_key_value(data: &str) -> Option<HashMap<u8, &str>> {
	let split: Vec<_> = data.split(':').collect();
	let mut parsed = HashMap::new();

	for chunk in split.chunks(2) {
		parsed.insert(chunk[0].parse().ok()?, chunk[1]);
	}

	Some(parsed)
}

pub trait Parse: Sized {
	fn from_str(data: &str) -> Option<Self>;
}

impl Parse for u32 {
	fn from_str(data: &str) -> Option<Self> {
		data.parse().ok()
	}
}

impl Parse for Gauntlet {
	fn from_str(data: &str) -> Option<Self> {
		let map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let mut level_ids = [0; 5];

		for (i, level_id) in map.get(&3)?.split(',').enumerate() {
			level_ids[i] = level_id.parse().ok()?;
		}

		Some(Self { id, level_ids })
	}
}

impl Parse for Level {
	fn from_str(data: &str) -> Option<Self> {
		let map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let name = map.get(&2)?.to_string();
		let description = decode::base64(map.get(&3)?).ok()?;

		Some(Self { id, name, description })
	}
}

impl Parse for LoginUser {
	fn from_str(data: &str) -> Option<Self> {
		let split = data.split_once(',')?;

		let id = split.1.parse().ok()?;
		let account_id = split.0.parse().ok()?;

		Some(Self { id, account_id })
	}
}

impl Parse for MapPack {
	fn from_str(data: &str) -> Option<Self> {
		let map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let name = map.get(&2)?.to_string();
		let mut level_ids = [0; 3];

		for (i, level_id) in map.get(&3)?.split(',').enumerate() {
			level_ids[i] = level_id.parse().ok()?;
		}

		Some(Self { id, name, level_ids })
	}
}

impl Parse for User {
	fn from_str(data: &str) -> Option<Self> {
		let map = parse_key_value(data)?;

		let id = map.get(&2)?.parse().ok()?;
		let account_id = map.get(&16)?.parse().ok()?;
		let username = map.get(&1)?.to_string();

		Some(Self {
			id,
			account_id,
			username,
		})
	}
}

impl<T: Parse> Parse for Vec<T> {
	fn from_str(data: &str) -> Option<Self> {
		let data = data.split_once('#')?.0;

		if data.is_empty() {
			Some(Vec::new())
		} else {
			data.split('|').map(T::from_str).collect()
		}
	}
}
