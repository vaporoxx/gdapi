//! Items related to map packs.

use crate::model::id::{LevelId, MapPackId};

/// Represents a map pack.
#[derive(Clone, Debug)]
pub struct MapPack {
	/// The ids of the map pack levels
	pub level_ids: [LevelId; 3],
	/// The id of the map pack
	pub map_pack_id: MapPackId,
	/// The name of the map pack
	pub name: String,
}
