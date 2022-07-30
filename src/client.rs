//! An asynchronous client implementation.

use std::sync::Arc;

use crate::crypto::{base64, gjp};
use crate::error::{Error, Result};
use crate::form;
use crate::http::{Endpoint, Http};
use crate::model::gauntlet::Gauntlet;
use crate::model::id::{AccountId, CommentId, GauntletId, LevelId};
use crate::model::level::Level;
use crate::model::map_pack::MapPack;
use crate::model::user::{LoginUser, User};

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
	pub async fn delete_account_comment(&self, id: CommentId) -> Result<()> {
		let auth = self.http.auth().ok_or(Error::NotLoggedIn)?;
		let form = form::delete_account_comment(&auth, id);

		self.http.post(Endpoint::DeleteAccComment, form).await
	}

	/// Gets the levels of a gauntlet by its id.
	pub async fn gauntlet(&self, id: GauntletId) -> Result<Vec<Level>> {
		self.http.post(Endpoint::GetLevels, form::gauntlet(id)).await
	}

	/// Gets all available gauntlets.
	pub async fn gauntlets(&self) -> Result<Vec<Gauntlet>> {
		self.http.post(Endpoint::GetGauntlets, form::gauntlets()).await
	}

	/// Gets a level by its id.
	pub async fn level(&self, id: LevelId) -> Result<Level> {
		self.http.post(Endpoint::DownloadLevel, form::level(id)).await
	}

	/// Gets a list of levels.
	pub async fn levels(&self, ids: &[LevelId]) -> Result<Vec<Level>> {
		let strings: Vec<_> = ids.iter().map(ToString::to_string).collect();
		let query = strings.join(",");

		self.http.post(Endpoint::GetLevels, form::levels(&query)).await
	}

	/// Logs in the client to get access to auth-only endpoints.
	pub async fn login(&self, username: &str, password: &str) -> Result<LoginUser> {
		let user: LoginUser = self
			.http
			.post(Endpoint::LoginAccount, form::login(username, password))
			.await?;

		self.http.set_auth(user.account_id, gjp::encode(password)?);

		Ok(user)
	}

	/// Gets all map packs of the provided page.
	pub async fn map_packs(&self, page: u8) -> Result<Vec<MapPack>> {
		self.http.post(Endpoint::GetMapPacks, form::map_packs(page)).await
	}

	/// Searches for a user.
	pub async fn search_user(&self, query: &str) -> Result<User> {
		self.http.post(Endpoint::GetUsers, form::search_user(query)).await
	}

	/// Uploads an account comment. Requires the client to be logged in.
	pub async fn upload_account_comment(&self, comment: &str) -> Result<CommentId> {
		let auth = self.http.auth().ok_or(Error::NotLoggedIn)?;
		let comment = base64::encode(comment);
		let form = form::upload_account_comment(&auth, &comment);

		self.http.post(Endpoint::UploadAccComment, form).await
	}

	/// Gets a user by its account id.
	pub async fn user(&self, id: AccountId) -> Result<User> {
		self.http.post(Endpoint::GetUserInfo, form::user(id)).await
	}
}
