use crate::crypto::decode;
use crate::error::Result;
use crate::model::gauntlet::Gauntlet;
use crate::model::level::{Copyability, Creator, Demon, Difficulty, Level, Rating};
use crate::model::map_pack::MapPack;
use crate::model::user::{LoginUser, User};
use crate::parser::error::Error;
use crate::parser::util::{List, Map};

pub trait Parse: Sized {
	fn parse(data: &str, remaining: Option<&str>) -> Result<Self>;
}

impl Parse for () {
	fn parse(_: &str, _: Option<&str>) -> Result<Self> {
		Ok(())
	}
}

impl Parse for u32 {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		Ok(data.parse().map_err(Error::from)?)
	}
}

impl Parse for Creator {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		let [user_id, username, account_id] = List::new(data, ':').strs()?;

		let account_id = account_id.parse().map_err(Error::from)?;
		let user_id = user_id.parse().map_err(Error::from)?;
		let username = username.into();

		Ok(Self {
			account_id,
			user_id,
			username,
		})
	}
}

impl Parse for Gauntlet {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		let map = Map::new(data, ':')?;

		let gauntlet_id = map.int(1)?;
		let level_ids = map.list(3, ',')?.ints()?;

		Ok(Self { gauntlet_id, level_ids })
	}
}

impl Parse for Level {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		let map = Map::new(data, ':')?;

		let creator_id = map.int(6)?;
		let description = map.base64(3)?;
		let level_id = map.int(1)?;
		let name = map.string(2)?;
		let stars = map.int(18)?;

		let copyability = match map.str(27) {
			Err(_) => None,
			Ok("0") => Some(Copyability::None),
			Ok(password) => {
				let password = decode::password(password)?;

				if password == 1 {
					Some(Copyability::Free)
				} else {
					Some(Copyability::Password(password - 1000000))
				}
			}
		};

		let difficulty = if map.bool(25)? {
			Difficulty::Auto
		} else if map.bool(17)? {
			match map.int(43)? {
				0 => Difficulty::Demon(Demon::Hard),
				3 => Difficulty::Demon(Demon::Easy),
				4 => Difficulty::Demon(Demon::Medium),
				5 => Difficulty::Demon(Demon::Insane),
				6 => Difficulty::Demon(Demon::Extreme),
				_ => return Err(Error::InvalidEnumValue(43))?,
			}
		} else {
			match map.int(9)? {
				0 => Difficulty::Unrated,
				10 => Difficulty::Easy,
				20 => Difficulty::Normal,
				30 => Difficulty::Hard,
				40 => Difficulty::Harder,
				50 => Difficulty::Insane,
				_ => return Err(Error::InvalidEnumValue(9))?,
			}
		};

		let rating = if stars == 0 {
			Rating::None
		} else if map.str(19)? == "0" {
			Rating::Rated
		} else {
			match map.int(42)? {
				0 => Rating::Featured,
				1 => Rating::Epic,
				2 => Rating::Legendary,
				3 => Rating::Mythic,
				_ => return Err(Error::InvalidEnumValue(42))?,
			}
		};

		Ok(Self {
			copyability,
			creator: None,
			creator_id,
			description,
			difficulty,
			level_id,
			name,
			rating,
			stars,
		})
	}
}

impl Parse for LoginUser {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		let [account_id, user_id] = List::new(data, ',').ints()?;

		Ok(Self { account_id, user_id })
	}
}

impl Parse for MapPack {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		let map = Map::new(data, ':')?;

		let level_ids = map.list(3, ',')?.ints()?;
		let map_pack_id = map.int(1)?;
		let name = map.string(2)?;

		Ok(Self {
			level_ids,
			map_pack_id,
			name,
		})
	}
}

impl Parse for User {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		let map = Map::new(data, ':')?;

		let account_id = map.int(16)?;
		let user_id = map.int(2)?;
		let username = map.string(1)?;

		Ok(Self {
			account_id,
			user_id,
			username,
		})
	}
}

impl Parse for Vec<Gauntlet> {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		List::new(data, '|').vec()
	}
}

impl Parse for Vec<Level> {
	fn parse(data: &str, remaining: Option<&str>) -> Result<Self> {
		let mut levels: Self = List::new(data, '|').vec()?;

		if let Some(remaining) = remaining {
			let next = List::new(remaining, '#').next()?;
			let creators: Vec<Creator> = List::new(next, '|').vec()?;

			for creator in creators {
				for level in &mut levels {
					if level.creator_id == creator.user_id {
						level.creator = Some(creator);
						break;
					}
				}
			}
		}

		Ok(levels)
	}
}

impl Parse for Vec<MapPack> {
	fn parse(data: &str, _: Option<&str>) -> Result<Self> {
		List::new(data, '|').vec()
	}
}
