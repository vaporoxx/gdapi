use crate::constants;
use crate::error::{Error, Result};
use crate::parse::Parse;
use reqwest::Client;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct Auth {
	pub account_id: u32,
	pub gjp: String,
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

	pub async fn post<T: Parse>(&self, endpoint: &str, form: impl Serialize) -> Result<T> {
		let url = format!("{}/{}.php", constants::BASE_URL, endpoint);
		let response = self.client.post(url).form(&form).send().await?.text().await?;

		if response == "-1" {
			Err(Error::InvalidRequest)
		} else {
			T::from_str(&response).ok_or(Error::ParseResponse)
		}
	}
}
