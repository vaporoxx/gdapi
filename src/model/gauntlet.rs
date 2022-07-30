//! Items related to gauntlets.

use super::id::{GauntletId, LevelId};

/// Represents a gauntlet.
#[derive(Clone, Copy, Debug)]
pub struct Gauntlet {
	/// The id of the gauntlet
	pub id: GauntletId,
	/// The ids of the gauntlet levels
	pub level_ids: [LevelId; 5],
}
