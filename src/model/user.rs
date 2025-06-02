//! Items related to users.

use crate::model::id::{AccountId, UserId};

/// Represents a partial user that you get when logging in.
#[derive(Clone, Copy, Debug)]
pub struct LoginUser {
	/// The account id of the user
	pub account_id: AccountId,
	/// The user id of the user
	pub user_id: UserId,
}

/// Represents a user.
#[derive(Clone, Debug)]
pub struct User {
	/// The account id of the user
	pub account_id: AccountId,
	/// The user id of the user
	pub user_id: UserId,
	/// The username of the user
	pub username: String,
}
