#[derive(Clone, Debug)]
pub struct Level {
	pub id: u32,
	pub name: String,
}

#[derive(Clone, Debug)]
pub struct LoginResponse {
	pub account_id: u32,
	pub user_id: u32,
}

#[derive(Clone, Debug)]
pub struct MapPack {
	pub id: u8,
	pub name: String,
}

#[derive(Clone, Debug)]
pub struct User {
	pub account_id: u32,
	pub username: String,
}
