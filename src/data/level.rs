//! Items related to levels.

use super::id::LevelId;

/// Represents an uploaded level.
#[derive(Clone, Debug)]
pub struct Level {
	/// The id of the level
	pub id: LevelId,
	/// The name of the level
	pub name: String,
	/// The description of the level
	pub description: String,
}
