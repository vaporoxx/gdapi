//! The error type and all related types.

use gdapi_crypto::error::Error as CryptoError;
use reqwest::Error as ReqwestError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

/// The error type used across the library.
#[derive(Debug)]
pub enum Error {
	/// An error returned by [`gdapi_crypto`]
	Crypto(CryptoError),
	/// The server received an invalid request
	InvalidRequest,
	/// The client needs to be logged in to use this method
	NotLoggedIn,
	/// The server response could not be parsed
	ParseResponse,
	/// An error returned by [`reqwest`]
	Reqwest(ReqwestError),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Crypto(error) => error.fmt(f),
			Self::InvalidRequest => write!(f, "server received an invalid request"),
			Self::NotLoggedIn => write!(f, "the client needs to be logged in to use this method"),
			Self::ParseResponse => write!(f, "server response could not be parsed"),
			Self::Reqwest(error) => error.fmt(f),
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Self::Crypto(error) => Some(error),
			Self::InvalidRequest => None,
			Self::NotLoggedIn => None,
			Self::ParseResponse => None,
			Self::Reqwest(error) => Some(error),
		}
	}
}

impl From<CryptoError> for Error {
	fn from(error: CryptoError) -> Self {
		Self::Crypto(error)
	}
}

impl From<ReqwestError> for Error {
	fn from(error: ReqwestError) -> Self {
		Self::Reqwest(error)
	}
}

/// The standard result type, but with the error set to [`Error`].
pub type Result<T> = StdResult<T, Error>;
