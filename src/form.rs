use crate::constants;
use serde::Serialize;

#[derive(Serialize)]
pub struct LevelForm {
	#[serde(rename = "levelID")]
	level_id: u32,
	secret: &'static str,
}

pub fn level(id: u32) -> LevelForm {
	LevelForm {
		level_id: id,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct SearchUserForm<'a> {
	str: &'a str,
	secret: &'static str,
}

pub fn search_user(username: &str) -> SearchUserForm {
	SearchUserForm {
		str: username,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct UserForm {
	#[serde(rename = "targetAccountID")]
	target_account_id: u32,
	secret: &'static str,
}

pub fn user(account_id: u32) -> UserForm {
	UserForm {
		target_account_id: account_id,
		secret: constants::SECRET,
	}
}
