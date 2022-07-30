//! Items related to levels.

use super::id::{AccountId, LevelId, UserId};

/// The author of a level.
#[derive(Clone, Debug)]
pub struct Author {
	/// The id of the user
	pub id: UserId,
	/// The account id of the user
	pub account_id: AccountId,
	/// The username of the user
	pub username: String,
}

/// The copy type of a level.
#[derive(Clone, Copy, Debug)]
pub enum CopyType {
	None,
	Free,
	Password(u32),
}

/// The demon difficulty of a level.
#[derive(Clone, Copy, Debug)]
pub enum DemonDifficulty {
	Easy,
	Medium,
	Hard,
	Insane,
	Extreme,
}

/// The difficulty of a level.
#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
	Unrated,
	Auto,
	Easy,
	Normal,
	Hard,
	Harder,
	Insane,
	Demon(DemonDifficulty),
}

/// Represents an uploaded level.
#[derive(Clone, Debug)]
pub struct Level {
	/// The id of the level
	pub id: LevelId,
	/// The name of the level
	pub name: String,
	/// The description of the level
	pub description: String,
	/// The id of the level's author
	pub author_id: UserId,
	/// The full author object
	pub author: Option<Author>,
	/// How many stars you get for beating the level
	pub stars: u8,
	/// The difficulty of the level
	pub difficulty: Difficulty,
	/// The rating of the level
	pub rating: Rating,
	/// The copy type of the level
	pub copy_type: Option<CopyType>,
}

/// The rating of a level.
#[derive(Clone, Copy, Debug)]
pub enum Rating {
	None,
	StarRated,
	Featured,
	Epic,
}
