//! An asynchronous client implementation.

use crate::data::{Gauntlet, Level, LoginUser, MapPack, User};
use crate::error::{Error, Result};
use crate::form;
use crate::http::{Endpoint, Http};
use gdapi_crypto::encode;
use std::sync::Arc;

/// The client used to make requests.
#[derive(Clone, Debug, Default)]
pub struct Client {
	http: Arc<Http>,
}

impl Client {
	/// Creates a new client.
	pub fn new() -> Self {
		Self::default()
	}

	/// Deletes an account comment. Requires the client to be logged in.
	pub async fn delete_account_comment(&self, comment_id: u32) -> Result<()> {
		let auth = self.http.auth().ok_or(Error::NotLoggedIn)?;
		let form = form::delete_account_comment(auth.account_id, &auth.gjp, comment_id);

		self.http.post(Endpoint::DeleteAccComment, form).await
	}

	/// Gets the levels of a gauntlet by its id.
	pub async fn gauntlet(&self, id: u8) -> Result<Vec<Level>> {
		self.http.post(Endpoint::GetLevels, form::gauntlet(id)).await
	}

	/// Gets all available gauntlets.
	pub async fn gauntlets(&self) -> Result<Vec<Gauntlet>> {
		self.http.post(Endpoint::GetGauntlets, form::gauntlets()).await
	}

	/// Gets a level by its id.
	pub async fn level(&self, id: u32) -> Result<Level> {
		self.http.post(Endpoint::DownloadLevel, form::level(id)).await
	}

	/// Gets a list of levels.
	pub async fn levels(&self, ids: &[u32]) -> Result<Vec<Level>> {
		self.http.post(Endpoint::GetLevels, form::levels(ids)).await
	}

	/// Logs in the client to get access to auth-only endpoints.
	pub async fn login(&self, username: &str, password: &str) -> Result<LoginUser> {
		let user: LoginUser = self
			.http
			.post(Endpoint::LoginAccount, form::login(username, password))
			.await?;

		self.http.set_auth(user.account_id, encode::gjp(password)?);

		Ok(user)
	}

	/// Gets all map packs of the provided page.
	pub async fn map_packs(&self, page: u8) -> Result<Vec<MapPack>> {
		self.http.post(Endpoint::GetMapPacks, form::map_packs(page)).await
	}

	/// Searches for a user by its username.
	pub async fn search_user(&self, username: &str) -> Result<User> {
		self.http.post(Endpoint::GetUsers, form::search_user(username)).await
	}

	/// Uploads an account comment. Requires the client to be logged in.
	pub async fn upload_account_comment(&self, comment: &str) -> Result<u32> {
		let auth = self.http.auth().ok_or(Error::NotLoggedIn)?;
		let comment = encode::base64(comment);
		let form = form::upload_account_comment(auth.account_id, &auth.gjp, &comment);

		self.http.post(Endpoint::UploadAccComment, form).await
	}

	/// Gets a user by its account id.
	pub async fn user(&self, account_id: u32) -> Result<User> {
		self.http.post(Endpoint::GetUserInfo, form::user(account_id)).await
	}
}
