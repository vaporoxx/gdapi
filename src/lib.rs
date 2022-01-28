//! A wrapper around the Geometry Dash API.

mod client;
mod constants;
mod form;
mod parser;

pub mod data;
pub mod error;

pub use client::Client;
