use crate::{constants, data::*, error::*, form, parsable::Parsable};
use gdapi_crypto::encode;
use reqwest::Client as ReqwestClient;
use serde::Serialize;

/// The client used to make requests.
#[derive(Clone, Debug, Default)]
pub struct Client {
	account_id: Option<u32>,
	gjp: Option<String>,
	inner: ReqwestClient,
}

impl Client {
	/// Creates a new client.
	pub fn new() -> Self {
		Self::default()
	}

	async fn request<T: Parsable>(&self, endpoint: &str, form: impl Serialize) -> Result<T> {
		let url = format!("{}/{}.php", constants::BASE_URL, endpoint);
		let response = self.inner.post(url).form(&form).send().await?.text().await?;

		if response == "-1" {
			Err(Error::InvalidRequest)
		} else {
			T::from_str(&response).ok_or(Error::ParseResponse)
		}
	}

	/// Gets the levels of a gauntlet by its id.
	pub async fn gauntlet(&self, id: u8) -> Result<Map<Level>> {
		self.request("getGJLevels21", form::gauntlet(id)).await
	}

	/// Gets all available gauntlets.
	pub async fn gauntlets(&self) -> Result<Map<Gauntlet>> {
		self.request("getGJGauntlets21", form::gauntlets()).await
	}

	/// Gets a level by its id.
	pub async fn level(&self, id: u32) -> Result<Level> {
		self.request("downloadGJLevel22", form::level(id)).await
	}

	/// Gets a list of levels.
	pub async fn levels(&self, ids: &[u32]) -> Result<Map<Level>> {
		self.request("getGJLevels21", form::levels(ids)).await
	}

	/// Logs in the client to get access to auth-only endpoints.
	pub async fn login(&mut self, username: &str, password: &str) -> Result<LoginUser> {
		let user: LoginUser = self
			.request("accounts/loginGJAccount", form::login(username, password))
			.await?;

		self.account_id = Some(user.account_id);
		self.gjp = Some(encode::gjp(password)?);

		Ok(user)
	}

	/// Gets all map packs of the provided page.
	pub async fn map_packs(&self, page: u8) -> Result<Map<MapPack>> {
		self.request("getGJMapPacks21", form::map_packs(page)).await
	}

	/// Searches for a user by its username.
	pub async fn search_user(&self, username: &str) -> Result<User> {
		self.request("getGJUsers20", form::search_user(username)).await
	}

	/// Uploads an account comment. Requires the client to be logged in.
	pub async fn upload_account_comment(&self, comment: &str) -> Result<u32> {
		let account_id = self.account_id.ok_or(Error::NotLoggedIn)?;
		let gjp = self.gjp.as_ref().ok_or(Error::NotLoggedIn)?;
		let comment = encode::base64(comment);
		let form = form::upload_account_comment(account_id, gjp, &comment);

		self.request("uploadGJAccComment20", form).await
	}

	/// Gets a user by its account id.
	pub async fn user(&self, account_id: u32) -> Result<User> {
		self.request("getGJUserInfo20", form::user(account_id)).await
	}
}
