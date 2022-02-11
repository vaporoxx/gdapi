use crate::data::*;
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

pub trait Parsable: Sized {
	fn from_str(data: &str) -> Option<Self>;
}

impl Parsable for u32 {
	fn from_str(data: &str) -> Option<Self> {
		data.parse().ok()
	}
}

impl Parsable for Gauntlet {
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

impl Parsable for Level {
	fn from_str(data: &str) -> Option<Self> {
		let map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let name = map.get(&2)?.to_string();
		let description = decode::base64(map.get(&3)?).ok()?;

		Some(Self { id, name, description })
	}
}

impl Parsable for LoginUser {
	fn from_str(data: &str) -> Option<Self> {
		let split = data.split_once(',')?;

		let id = split.1.parse().ok()?;
		let account_id = split.0.parse().ok()?;

		Some(Self { id, account_id })
	}
}

impl Parsable for MapPack {
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

impl Parsable for User {
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

impl<T: Identifiable + Parsable> Parsable for Map<T> {
	fn from_str(data: &str) -> Option<Self> {
		let mut map = Map::new();

		for split in data.split_once('#')?.0.split('|') {
			if let Some(parsed) = T::from_str(split) {
				map.insert(parsed.id(), parsed);
			}
		}

		Some(map)
	}
}
