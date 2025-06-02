use std::sync::OnceLock;

use reqwest::Client;
use serde::Serialize;

use crate::error::{Error, Result};
use crate::model::id::AccountId;
use crate::parser::parse::Parse;

#[derive(Debug)]
pub struct Auth {
	pub account_id: AccountId,
	pub gjp2: String,
}

pub enum Endpoint {
	DeleteAccComment,
	DownloadLevel,
	GetGauntlets,
	GetLevels,
	GetMapPacks,
	GetUserInfo,
	GetUsers,
	LoginAccount,
	UploadAccComment,
}

impl Endpoint {
	const fn endpoint(&self) -> &'static str {
		match self {
			Self::DeleteAccComment => "deleteGJAccComment20",
			Self::DownloadLevel => "downloadGJLevel22",
			Self::GetGauntlets => "getGJGauntlets21",
			Self::GetLevels => "getGJLevels21",
			Self::GetMapPacks => "getGJMapPacks21",
			Self::GetUserInfo => "getGJUserInfo20",
			Self::GetUsers => "getGJUsers20",
			Self::LoginAccount => "accounts/loginGJAccount",
			Self::UploadAccComment => "uploadGJAccComment20",
		}
	}
}

#[derive(Debug, Default)]
pub struct Http {
	auth: OnceLock<Auth>,
	client: Client,
}

impl Http {
	const API_BASE: &str = "https://www.boomlings.com/database";

	pub fn auth(&self) -> Result<&Auth> {
		self.auth.get().ok_or(Error::NotLoggedIn)
	}

	pub fn authenticate(&self, account_id: AccountId, gjp2: String) -> Result<()> {
		self.auth.set(Auth { account_id, gjp2 }).or(Err(Error::AlreadyLoggedIn))
	}

	pub async fn post<T: Parse>(&self, endpoint: Endpoint, form: impl Serialize) -> Result<T> {
		let url = format!("{}/{}.php", Self::API_BASE, endpoint.endpoint());

		let response = self.client.post(url).form(&form).send().await?;
		let data = response.error_for_status()?.text().await?;

		if data.starts_with('-') {
			return Err(Error::InvalidRequest(data.parse().unwrap_or_default()));
		}

		if let Some((data, remaining)) = data.split_once('#') {
			T::parse(data, Some(remaining))
		} else {
			T::parse(&data, None)
		}
	}
}
