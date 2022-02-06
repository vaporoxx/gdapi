//! A wrapper around the Geometry Dash API.

mod client;
mod constants;
mod form;
mod parsable;

pub mod data;
pub mod error;

pub use client::Client;
