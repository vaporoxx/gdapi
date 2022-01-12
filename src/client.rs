use crate::{constants, data::*, error::*, form, parser::*};
use reqwest::Client as ReqwestClient;
use serde::Serialize;

#[derive(Clone, Debug, Default)]
pub struct Client {
	inner: ReqwestClient,
}

impl Client {
	pub fn new() -> Self {
		Self::default()
	}

	async fn request(&self, endpoint: &str, form: impl Serialize) -> Result<APIData> {
		let url = format!("{}/{}.php", constants::BASE_URL, endpoint);
		let response = self.inner.post(url).form(&form).send().await?.text().await?;

		if response == "-1" {
			Err(Error::InvalidRequest)
		} else {
			parse_data(&response).ok_or(Error::ParseResponse)
		}
	}

	pub async fn level(&self, id: u32) -> Result<Level> {
		let mut data = self.request("downloadGJLevel22", form::level(id)).await?;
		let name = data.remove(&2).unwrap();

		Ok(Level { id, name })
	}

	pub async fn user(&self, id: u32) -> Result<User> {
		let mut data = self.request("getGJUserInfo20", form::user(id)).await?;
		let name = data.remove(&1).unwrap();

		Ok(User { id, name })
	}

	pub async fn search_user(&self, name: &str) -> Result<User> {
		let mut data = self.request("getGJUsers20", form::search_user(name)).await?;
		let name = data.remove(&1).unwrap();
		let id = data.remove(&16).unwrap().parse().unwrap();

		Ok(User { id, name })
	}
}
