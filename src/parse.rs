use std::collections::HashMap;

use crate::crypto::{base64, password};
use crate::model::gauntlet::Gauntlet;
use crate::model::level::{Author, CopyType, DemonDifficulty, Difficulty, Level, Rating};
use crate::model::map_pack::MapPack;
use crate::model::user::{LoginUser, User};

fn parse_key_value(data: &str) -> Option<HashMap<u8, &str>> {
	let split: Vec<_> = data.split(':').collect();
	let mut parsed = HashMap::new();

	for chunk in split.chunks(2) {
		parsed.insert(chunk[0].parse().ok()?, chunk[1]);
	}

	Some(parsed)
}

fn parse_vec<T: Parse>(data: &str) -> Option<Vec<T>> {
	if data.is_empty() {
		Some(Vec::new())
	} else {
		data.split('|').map(|data| T::parse(data, None)).collect()
	}
}

pub trait Parse: Sized {
	fn parse(data: &str, remaining: Option<&str>) -> Option<Self>;
}

impl Parse for () {
	fn parse(_: &str, _: Option<&str>) -> Option<Self> {
		Some(())
	}
}

impl Parse for u32 {
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
		data.parse().ok()
	}
}

impl Parse for Author {
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
		let mut split = data.split(':');

		let id = split.next()?.parse().ok()?;
		let username = split.next()?.to_string();
		let account_id = split.next()?.parse().ok()?;

		Some(Self {
			id,
			account_id,
			username,
		})
	}
}

impl Parse for Gauntlet {
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
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
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
		let map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let name = map.get(&2)?.to_string();
		let description = base64::decode(map.get(&3)?).ok()?;

		let author_id = map.get(&6)?.parse().ok()?;
		let stars = map.get(&18)?.parse().ok()?;

		let difficulty = if !map.get(&25)?.is_empty() {
			Difficulty::Auto
		} else if !map.get(&17)?.is_empty() {
			match map.get(&43)?.parse().ok()? {
				0 => Difficulty::Demon(DemonDifficulty::Hard),
				3 => Difficulty::Demon(DemonDifficulty::Easy),
				4 => Difficulty::Demon(DemonDifficulty::Medium),
				5 => Difficulty::Demon(DemonDifficulty::Insane),
				6 => Difficulty::Demon(DemonDifficulty::Extreme),
				_ => return None,
			}
		} else {
			match map.get(&9)?.parse().ok()? {
				0 => Difficulty::Unrated,
				10 => Difficulty::Easy,
				20 => Difficulty::Normal,
				30 => Difficulty::Hard,
				40 => Difficulty::Harder,
				50 => Difficulty::Insane,
				_ => return None,
			}
		};

		let rating = if stars == 0 {
			Rating::None
		} else if map.get(&42)? != &"0" {
			Rating::Epic
		} else if map.get(&19)? != &"0" {
			Rating::Featured
		} else {
			Rating::StarRated
		};

		let copy_type = match map.get(&27) {
			Some(&"0") => Some(CopyType::None),
			Some(password) => {
				let password = password::decode(password).ok()?;

				if password == 1 {
					Some(CopyType::Free)
				} else {
					Some(CopyType::Password(password % 1000000))
				}
			}
			None => None,
		};

		Some(Self {
			id,
			name,
			description,
			author_id,
			author: None,
			stars,
			difficulty,
			rating,
			copy_type,
		})
	}
}

impl Parse for LoginUser {
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
		let split = data.split_once(',')?;

		let id = split.1.parse().ok()?;
		let account_id = split.0.parse().ok()?;

		Some(Self { id, account_id })
	}
}

impl Parse for MapPack {
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
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
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
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

impl Parse for Vec<Gauntlet> {
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
		parse_vec(data)
	}
}

impl Parse for Vec<Level> {
	fn parse(data: &str, remaining: Option<&str>) -> Option<Self> {
		let author_data = remaining?.split_once('#')?.0;

		let mut levels: Vec<Level> = parse_vec(data)?;
		let authors: Vec<Author> = parse_vec(author_data)?;

		for author in authors {
			let author_id = author.id;

			levels.iter_mut().find(|e| e.author_id == author_id)?.author = Some(author);
		}

		Some(levels)
	}
}

impl Parse for Vec<MapPack> {
	fn parse(data: &str, _: Option<&str>) -> Option<Self> {
		parse_vec(data)
	}
}
