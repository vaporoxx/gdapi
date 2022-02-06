/// Represents a gauntlet.
#[derive(Clone, Copy, Debug)]
pub struct Gauntlet {
	/// The id of the gauntlet
	pub id: u8,
	/// The ids of the gauntlet levels
	pub level_ids: [u32; 5],
}

/// Represents an uploaded level.
#[derive(Clone, Debug)]
pub struct Level {
	/// The id of the level
	pub id: u32,
	/// The name of the level
	pub name: String,
	/// The description of the level
	pub description: String,
}

/// Represents a map pack.
#[derive(Clone, Debug)]
pub struct MapPack {
	/// The id of the map pack
	pub id: u8,
	/// The name of the map pack
	pub name: String,
	/// The ids of the map pack levels
	pub level_ids: [u32; 3],
}

/// Represents a partial user that you get when logging in.
#[derive(Clone, Copy, Debug)]
pub struct PartialUser {
	/// The id of the logged in user
	pub id: u32,
	/// The account id of the logged in user
	pub account_id: u32,
}

/// Represents a user.
#[derive(Clone, Debug)]
pub struct User {
	/// The id of the user
	pub id: u32,
	/// The account id of the user
	pub account_id: u32,
	/// The username of the user
	pub username: String,
}
