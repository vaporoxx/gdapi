use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::Arc;

use parking_lot::Mutex;
use reqwest::Client;
use serde::Serialize;

use crate::constants;
use crate::error::{Error, Result};
use crate::parse::Parse;

#[derive(Debug)]
pub struct Auth {
	pub account_id: u32,
	pub gjp: String,
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

impl Display for Endpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let endpoint = match self {
			Endpoint::DeleteAccComment => "deleteGJAccComment20",
			Endpoint::DownloadLevel => "downloadGJLevel22",
			Endpoint::GetGauntlets => "getGJGauntlets21",
			Endpoint::GetLevels => "getGJLevels21",
			Endpoint::GetMapPacks => "getGJMapPacks21",
			Endpoint::GetUserInfo => "getGJUserInfo20",
			Endpoint::GetUsers => "getGJUsers20",
			Endpoint::LoginAccount => "accounts/loginGJAccount",
			Endpoint::UploadAccComment => "uploadGJAccComment20",
		};

		write!(f, "{}/{}.php", constants::BASE_URL, endpoint)
	}
}

#[derive(Debug, Default)]
pub struct Http {
	auth: Mutex<Option<Arc<Auth>>>,
	client: Client,
}

impl Http {
	pub fn auth(&self) -> Option<Arc<Auth>> {
		self.auth.lock().clone()
	}

	pub fn set_auth(&self, account_id: u32, gjp: String) {
		*self.auth.lock() = Some(Arc::new(Auth { account_id, gjp }));
	}

	pub async fn post<T: Parse>(&self, endpoint: Endpoint, form: impl Serialize) -> Result<T> {
		let url = endpoint.to_string();
		let response = self.client.post(url).form(&form).send().await?.text().await?;

		if response == "-1" {
			Err(Error::InvalidRequest)
		} else {
			T::parse(&response).ok_or(Error::ParseResponse)
		}
	}
}
