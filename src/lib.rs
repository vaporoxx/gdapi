//! A wrapper around the Geometry Dash API.

#![doc(
	html_favicon_url = "https://raw.githubusercontent.com/gdapi-rs/.github/main/assets/icon.png",
	html_logo_url = "https://raw.githubusercontent.com/gdapi-rs/.github/main/assets/icon.png"
)]

mod constants;
mod form;
mod http;
mod parse;

pub mod client;
pub mod data;
pub mod error;
