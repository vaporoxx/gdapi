//! Items related to levels.

use crate::model::id::{AccountId, LevelId, UserId};

/// Represents the copyability of a level.
#[derive(Clone, Copy, Debug)]
pub enum Copyability {
	None,
	Free,
	Password(u32),
}

/// Represents the creator of a level.
#[derive(Clone, Debug)]
pub struct Creator {
	/// The account id of the creator
	pub account_id: AccountId,
	/// The user id of the creator
	pub user_id: UserId,
	/// The username of the creator
	pub username: String,
}

/// Represents the specific demon difficulty of a level.
#[derive(Clone, Copy, Debug)]
pub enum Demon {
	Easy,
	Medium,
	Hard,
	Insane,
	Extreme,
}

/// Represents the difficulty of a level.
#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
	Unrated,
	Auto,
	Easy,
	Normal,
	Hard,
	Harder,
	Insane,
	Demon(Demon),
}

/// Represents a level.
#[derive(Clone, Debug)]
pub struct Level {
	/// The copyability of the level
	pub copyability: Option<Copyability>,
	/// The creator of the level
	pub creator: Option<Creator>,
	/// The user id of the level creator
	pub creator_id: UserId,
	/// The description of the level
	pub description: String,
	/// The difficulty of the level
	pub difficulty: Difficulty,
	/// The id of the level
	pub level_id: LevelId,
	/// The name of the level
	pub name: String,
	/// The rating of the level
	pub rating: Rating,
	/// The stars of the level
	pub stars: u8,
}

/// Represents the rating of a level.
#[derive(Clone, Copy, Debug)]
pub enum Rating {
	None,
	Rated,
	Featured,
	Epic,
	Legendary,
	Mythic,
}
