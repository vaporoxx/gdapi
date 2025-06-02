use serde::Serialize;

use crate::http::Auth;
use crate::model::id::{AccountId, CommentId, GauntletId, LevelId};

mod secrets {
	pub const ACCOUNT: &str = "Wmfv3899gc9";
	pub const COMMON: &str = "Wmfd2893gb7";
}

#[derive(Serialize)]
pub struct DeleteAccountCommentForm<'a> {
	#[serde(rename = "accountID")]
	account_id: AccountId,
	#[serde(rename = "commentID")]
	comment_id: CommentId,
	gjp2: &'a str,
	secret: &'static str,
	#[serde(rename = "targetAccountID")]
	target_account_id: AccountId,
}

pub const fn delete_account_comment(auth: &Auth, id: CommentId) -> DeleteAccountCommentForm {
	DeleteAccountCommentForm {
		account_id: auth.account_id,
		comment_id: id,
		gjp2: auth.gjp2.as_str(),
		secret: secrets::COMMON,
		target_account_id: auth.account_id,
	}
}

#[derive(Serialize)]
pub struct GauntletForm {
	gauntlet: GauntletId,
	secret: &'static str,
}

pub const fn gauntlet(id: GauntletId) -> GauntletForm {
	GauntletForm {
		gauntlet: id,
		secret: secrets::COMMON,
	}
}

#[derive(Serialize)]
pub struct GauntletsForm {
	secret: &'static str,
	special: u8,
}

pub const fn gauntlets() -> GauntletsForm {
	GauntletsForm {
		secret: secrets::COMMON,
		special: 1,
	}
}

#[derive(Serialize)]
pub struct LevelForm {
	#[serde(rename = "levelID")]
	level_id: LevelId,
	secret: &'static str,
}

pub const fn level(id: LevelId) -> LevelForm {
	LevelForm {
		level_id: id,
		secret: secrets::COMMON,
	}
}

#[derive(Serialize)]
pub struct LevelsForm<'a> {
	#[serde(rename = "type")]
	kind: u8,
	secret: &'static str,
	str: &'a str,
}

pub const fn levels(query: &str) -> LevelsForm {
	LevelsForm {
		kind: 10,
		secret: secrets::COMMON,
		str: query,
	}
}

#[derive(Serialize)]
pub struct LoginForm<'a> {
	gjp2: &'a str,
	secret: &'static str,
	udid: &'static str,
	#[serde(rename = "userName")]
	username: &'a str,
}

pub const fn login<'a>(username: &'a str, gjp2: &'a str) -> LoginForm<'a> {
	LoginForm {
		gjp2,
		secret: secrets::ACCOUNT,
		udid: env!("CARGO_PKG_NAME"),
		username,
	}
}

#[derive(Serialize)]
pub struct MapPacksForm {
	page: u8,
	secret: &'static str,
}

pub const fn map_packs(page: u8) -> MapPacksForm {
	MapPacksForm {
		page,
		secret: secrets::COMMON,
	}
}

#[derive(Serialize)]
pub struct SearchUserForm<'a> {
	secret: &'static str,
	str: &'a str,
}

pub const fn search_user(query: &str) -> SearchUserForm {
	SearchUserForm {
		secret: secrets::COMMON,
		str: query,
	}
}

#[derive(Serialize)]
pub struct UploadAccountCommentForm<'a> {
	#[serde(rename = "accountID")]
	account_id: AccountId,
	comment: &'a str,
	gjp2: &'a str,
	secret: &'static str,
}

pub const fn upload_account_comment<'a>(auth: &'a Auth, comment: &'a str) -> UploadAccountCommentForm<'a> {
	UploadAccountCommentForm {
		account_id: auth.account_id,
		comment,
		gjp2: auth.gjp2.as_str(),
		secret: secrets::COMMON,
	}
}

#[derive(Serialize)]
pub struct UserForm {
	secret: &'static str,
	#[serde(rename = "targetAccountID")]
	target_account_id: AccountId,
}

pub const fn user(id: AccountId) -> UserForm {
	UserForm {
		secret: secrets::COMMON,
		target_account_id: id,
	}
}
