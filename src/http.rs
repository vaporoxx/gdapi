use crate::constants;
use crate::error::{Error, Result};
use crate::parse::Parse;
use reqwest::Client;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug)]
pub struct Auth {
	pub account_id: u32,
	pub gjp: String,
}

pub enum Endpoint {
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

#[derive(Clone, Debug, Default)]
pub struct HttpManager {
	auth: Option<Auth>,
	client: Client,
}

impl HttpManager {
	pub fn auth(&self) -> Option<&Auth> {
		self.auth.as_ref()
	}

	pub fn set_auth(&mut self, account_id: u32, gjp: String) {
		self.auth = Some(Auth { account_id, gjp });
	}

	pub async fn post<T: Parse>(&self, endpoint: Endpoint, form: impl Serialize) -> Result<T> {
		let url = endpoint.to_string();
		let response = self.client.post(url).form(&form).send().await?.text().await?;

		if response == "-1" {
			Err(Error::InvalidRequest)
		} else {
			T::from_str(&response).ok_or(Error::ParseResponse)
		}
	}
}
