//! A wrapper around the Geometry Dash API.

#![doc(
	html_favicon_url = "https://raw.githubusercontent.com/gdapi-rs/gdapi/main/assets/icon.png",
	html_logo_url = "https://raw.githubusercontent.com/gdapi-rs/gdapi/main/assets/icon.png"
)]

mod constants;
mod crypto;
mod form;
mod http;
mod parse;

pub mod client;
pub mod error;
pub mod model;
