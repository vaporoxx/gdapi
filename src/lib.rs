//! A wrapper around the Geometry Dash API.

mod constants;
mod form;
mod http;
mod parse;

pub mod data;
pub mod error;

pub use http::Client;
