use crate::data::*;
use std::collections::HashMap;

fn parse_key_value(data: &str) -> Option<HashMap<u8, String>> {
	let split: Vec<_> = data.split(':').collect();
	let mut parsed = HashMap::new();

	for chunk in split.chunks(2) {
		parsed.insert(chunk[0].parse().ok()?, chunk[1].into());
	}

	Some(parsed)
}

pub trait APIData: Sized {
	fn parse_data(data: &str) -> Option<Self>;
}

impl APIData for u32 {
	fn parse_data(data: &str) -> Option<Self> {
		data.parse().ok()
	}
}

impl APIData for Gauntlet {
	fn parse_data(data: &str) -> Option<Self> {
		let map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let mut level_ids = [0; 5];

		for (i, level_id) in map.get(&3)?.split(',').enumerate() {
			level_ids[i] = level_id.parse().ok()?;
		}

		Some(Self { id, level_ids })
	}
}

impl APIData for Level {
	fn parse_data(data: &str) -> Option<Self> {
		let mut map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let name = map.remove(&2)?;

		Some(Self { id, name })
	}
}

impl APIData for LoginResponse {
	fn parse_data(data: &str) -> Option<Self> {
		let split = data.split_once(',')?;

		let account_id = split.0.parse().ok()?;
		let user_id = split.1.parse().ok()?;

		Some(Self { account_id, user_id })
	}
}

impl APIData for MapPack {
	fn parse_data(data: &str) -> Option<Self> {
		let mut map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let name = map.remove(&2)?;

		Some(Self { id, name })
	}
}

impl APIData for User {
	fn parse_data(data: &str) -> Option<Self> {
		let mut map = parse_key_value(data)?;

		let account_id = map.get(&16)?.parse().ok()?;
		let username = map.remove(&1)?;

		Some(Self { account_id, username })
	}
}

impl<T: APIData> APIData for Vec<T> {
	fn parse_data(data: &str) -> Option<Self> {
		Some(data.split_once('#')?.0.split('|').filter_map(T::parse_data).collect())
	}
}
