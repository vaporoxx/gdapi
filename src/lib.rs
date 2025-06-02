#![deny(clippy::all, clippy::nursery)]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/vaporoxx/gdapi/main/assets/icon.png")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/vaporoxx/gdapi/main/assets/icon.png")]

mod crypto;
mod form;
mod http;
mod parser;

pub mod client;
pub mod error;
pub mod model;
