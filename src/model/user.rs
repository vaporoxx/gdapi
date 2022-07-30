//! Items related to users.

use super::id::{AccountId, UserId};

/// Represents a partial user that you get when logging in.
#[derive(Clone, Copy, Debug)]
pub struct LoginUser {
	/// The id of the user
	pub id: UserId,
	/// The account id of the user
	pub account_id: AccountId,
}

/// Represents a user.
#[derive(Clone, Debug)]
pub struct User {
	/// The id of the user
	pub id: UserId,
	/// The account id of the user
	pub account_id: AccountId,
	/// The username of the user
	pub username: String,
}
