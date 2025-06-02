//! Items related to gauntlets.

use crate::model::id::{GauntletId, LevelId};

/// Represents a gauntlet.
#[derive(Clone, Copy, Debug)]
pub struct Gauntlet {
	/// The id of the gauntlet
	pub gauntlet_id: GauntletId,
	/// The ids of the gauntlet levels
	pub level_ids: [LevelId; 5],
}
