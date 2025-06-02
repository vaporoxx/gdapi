//! The error type and all related types.

use std::{error, fmt, result};

use reqwest::Error as ReqwestError;

pub use crate::crypto::error::Error as CryptoError;
pub use crate::parser::error::Error as ParserError;

/// The error type used across the library.
#[derive(Debug)]
pub enum Error {
	/// The client is already logged in
	AlreadyLoggedIn,
	/// A wrapper around a [`CryptoError`]
	Crypto(CryptoError),
	/// The server rejected the request
	InvalidRequest(i8),
	/// The client needs to be logged in
	NotLoggedIn,
	/// A wrapper around a [`ParserError`]
	Parser(ParserError),
	/// A wrapper around a [`ReqwestError`]
	Reqwest(ReqwestError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::AlreadyLoggedIn => write!(f, "client is already logged in"),
			Self::Crypto(error) => error.fmt(f),
			Self::InvalidRequest(code) => write!(f, "server rejected the request (status code {code})"),
			Self::NotLoggedIn => write!(f, "client needs to be logged in"),
			Self::Parser(error) => error.fmt(f),
			Self::Reqwest(error) => error.fmt(f),
		}
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Self::AlreadyLoggedIn => None,
			Self::Crypto(error) => Some(error),
			Self::InvalidRequest(_) => None,
			Self::NotLoggedIn => None,
			Self::Parser(error) => Some(error),
			Self::Reqwest(error) => Some(error),
		}
	}
}

impl From<CryptoError> for Error {
	fn from(error: CryptoError) -> Self {
		Self::Crypto(error)
	}
}

impl From<ParserError> for Error {
	fn from(error: ParserError) -> Self {
		Self::Parser(error)
	}
}

impl From<ReqwestError> for Error {
	fn from(error: ReqwestError) -> Self {
		Self::Reqwest(error)
	}
}

/// The standard result type, but with the error defaulting to [`Error`].
pub type Result<T, E = Error> = result::Result<T, E>;
