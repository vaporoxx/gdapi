//! Items related to map packs.

use super::id::{LevelId, MapPackId};

/// Represents a map pack.
#[derive(Clone, Debug)]
pub struct MapPack {
	/// The id of the map pack
	pub id: MapPackId,
	/// The name of the map pack
	pub name: String,
	/// The ids of the map pack levels
	pub level_ids: [LevelId; 3],
}
