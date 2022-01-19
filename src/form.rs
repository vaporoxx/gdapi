use crate::constants;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GauntletsForm {
	special: u8,
	secret: &'static str,
}

pub fn gauntlets() -> GauntletsForm {
	GauntletsForm {
		special: 1,
		secret: constants::SECRET,
	}
}

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
pub struct LoginForm<'a> {
	#[serde(rename = "userName")]
	username: &'a str,
	password: &'a str,
	udid: Uuid,
	secret: &'static str,
}

pub fn login<'a>(username: &'a str, password: &'a str) -> LoginForm<'a> {
	LoginForm {
		username,
		password,
		udid: Uuid::new_v4(),
		secret: constants::LOGIN_SECRET,
	}
}

#[derive(Serialize)]
pub struct MapPacksForm {
	page: Option<u8>,
	secret: &'static str,
}

pub fn map_packs(page: Option<u8>) -> MapPacksForm {
	MapPacksForm {
		page,
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
