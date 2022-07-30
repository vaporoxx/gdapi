use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::string::FromUtf8Error;

use base64::DecodeError;

/// An error returned when trying to decode or encode a value.
#[derive(Debug)]
pub enum Error {
	/// An error returned by [`base64::decode_config`]
	Decode(DecodeError),
	/// An error returned by [`String::from_utf8`]
	FromUtf8(FromUtf8Error),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Decode(error) => error.fmt(f),
			Self::FromUtf8(error) => error.fmt(f),
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Self::Decode(error) => Some(error),
			Self::FromUtf8(error) => Some(error),
		}
	}
}

impl From<DecodeError> for Error {
	fn from(error: DecodeError) -> Self {
		Self::Decode(error)
	}
}

impl From<FromUtf8Error> for Error {
	fn from(error: FromUtf8Error) -> Self {
		Self::FromUtf8(error)
	}
}

pub type Result<T> = StdResult<T, Error>;
