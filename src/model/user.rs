//! Items related to users.

use crate::model::id::{AccountId, UserId};

/// Represents a partial user that you get when logging in.
#[derive(Clone, Copy, Debug)]
pub struct LoginUser {
	/// The account id of the user.
	pub account_id: AccountId,
	/// The user id of the user.
	pub user_id: UserId,
}

/// Represents a user.
#[derive(Clone, Debug)]
pub struct User {
	/// The account id of the user.
	pub account_id: AccountId,
	/// The creator point count of the user.
	pub creator_points: u32,
	/// The demon count of the user.
	pub demons: u32,
	/// The diamond count of the user.
	pub diamonds: u32,
	/// The moon count of the user.
	pub moons: u32,
	/// The secret coin count of the user.
	pub secret_coins: u8,
	/// The star count of the user.
	pub stars: u32,
	/// The user coin count of the user.
	pub user_coins: u32,
	/// The user id of the user.
	pub user_id: UserId,
	/// The username of the user.
	pub username: String,
}
