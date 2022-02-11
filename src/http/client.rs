use crate::{data::*, error::*, form, http::HttpManager};
use gdapi_crypto::encode;

/// The client used to make requests.
#[derive(Clone, Debug, Default)]
pub struct Client {
	http: HttpManager,
}

impl Client {
	/// Creates a new client.
	pub fn new() -> Self {
		Self::default()
	}

	/// Gets the levels of a gauntlet by its id.
	pub async fn gauntlet(&self, id: u8) -> Result<Map<Level>> {
		self.http.post("getGJLevels21", form::gauntlet(id)).await
	}

	/// Gets all available gauntlets.
	pub async fn gauntlets(&self) -> Result<Map<Gauntlet>> {
		self.http.post("getGJGauntlets21", form::gauntlets()).await
	}

	/// Gets a level by its id.
	pub async fn level(&self, id: u32) -> Result<Level> {
		self.http.post("downloadGJLevel22", form::level(id)).await
	}

	/// Gets a list of levels.
	pub async fn levels(&self, ids: &[u32]) -> Result<Map<Level>> {
		self.http.post("getGJLevels21", form::levels(ids)).await
	}

	/// Logs in the client to get access to auth-only endpoints.
	pub async fn login(&mut self, username: &str, password: &str) -> Result<LoginUser> {
		let user: LoginUser = self
			.http
			.post("accounts/loginGJAccount", form::login(username, password))
			.await?;

		self.http.set_auth(user.account_id, encode::gjp(password)?);

		Ok(user)
	}

	/// Gets all map packs of the provided page.
	pub async fn map_packs(&self, page: u8) -> Result<Map<MapPack>> {
		self.http.post("getGJMapPacks21", form::map_packs(page)).await
	}

	/// Searches for a user by its username.
	pub async fn search_user(&self, username: &str) -> Result<User> {
		self.http.post("getGJUsers20", form::search_user(username)).await
	}

	/// Uploads an account comment. Requires the client to be logged in.
	pub async fn upload_account_comment(&self, comment: &str) -> Result<u32> {
		let auth = self.http.auth().ok_or(Error::NotLoggedIn)?;
		let comment = encode::base64(comment);
		let form = form::upload_account_comment(auth.account_id, &auth.gjp, &comment);

		self.http.post("uploadGJAccComment20", form).await
	}

	/// Gets a user by its account id.
	pub async fn user(&self, account_id: u32) -> Result<User> {
		self.http.post("getGJUserInfo20", form::user(account_id)).await
	}
}
