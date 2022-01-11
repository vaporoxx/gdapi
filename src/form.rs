use crate::constants;
use serde::Serialize;

#[derive(Serialize)]
pub struct LevelForm {
	#[serde(rename = "levelID")]
	level_id: u32,
	secret: String,
}

pub fn level(id: u32) -> LevelForm {
	LevelForm {
		level_id: id,
		secret: constants::SECRET.into(),
	}
}

#[derive(Serialize)]
pub struct UserForm {
	#[serde(rename = "targetAccountID")]
	target_account_id: u32,
	secret: String,
}

pub fn user(id: u32) -> UserForm {
	UserForm {
		target_account_id: id,
		secret: constants::SECRET.into(),
	}
}
