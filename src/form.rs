use serde::Serialize;
use uuid::Uuid;

use crate::constants;
use crate::http::Auth;
use crate::model::id::{AccountId, CommentId, GauntletId, LevelId};

#[derive(Serialize)]
pub struct DeleteAccountCommentForm<'a> {
	#[serde(rename = "accountID")]
	account_id: AccountId,
	gjp: &'a str,
	#[serde(rename = "commentID")]
	comment_id: CommentId,
	secret: &'static str,
}

pub fn delete_account_comment(auth: &Auth, id: CommentId) -> DeleteAccountCommentForm {
	DeleteAccountCommentForm {
		account_id: auth.account_id,
		gjp: &auth.gjp,
		comment_id: id,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct GauntletForm {
	gauntlet: GauntletId,
	secret: &'static str,
}

pub fn gauntlet(id: GauntletId) -> GauntletForm {
	GauntletForm {
		gauntlet: id,
		secret: constants::SECRET,
	}
}

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
	level_id: LevelId,
	secret: &'static str,
}

pub fn level(id: LevelId) -> LevelForm {
	LevelForm {
		level_id: id,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct LevelsForm<'a> {
	str: &'a str,
	r#type: u8,
	secret: &'static str,
}

pub fn levels(query: &str) -> LevelsForm {
	LevelsForm {
		str: query,
		r#type: 10,
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
	page: u8,
	secret: &'static str,
}

pub fn map_packs(page: u8) -> MapPacksForm {
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

pub fn search_user(query: &str) -> SearchUserForm {
	SearchUserForm {
		str: query,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct UploadAccountCommentForm<'a> {
	#[serde(rename = "accountID")]
	account_id: AccountId,
	gjp: &'a str,
	comment: &'a str,
	secret: &'static str,
}

pub fn upload_account_comment<'a>(auth: &'a Auth, comment: &'a str) -> UploadAccountCommentForm<'a> {
	UploadAccountCommentForm {
		account_id: auth.account_id,
		gjp: &auth.gjp,
		comment,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct UserForm {
	#[serde(rename = "targetAccountID")]
	target_account_id: AccountId,
	secret: &'static str,
}

pub fn user(id: AccountId) -> UserForm {
	UserForm {
		target_account_id: id,
		secret: constants::SECRET,
	}
}
