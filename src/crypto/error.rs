use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;
use std::result::Result as StdResult;
use std::string::FromUtf8Error;

use base64::DecodeError;

/// An error returned when trying to decode or encode a value.
#[derive(Debug)]
pub enum Error {
	/// An error returned by [`base64`]
	Decode(DecodeError),
	/// An error returned by [`String::from_utf8`]
	FromUtf8(FromUtf8Error),
	/// An error returned when parsing an integer fails
	ParseInt(ParseIntError),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Decode(error) => error.fmt(f),
			Self::FromUtf8(error) => error.fmt(f),
			Self::ParseInt(error) => error.fmt(f),
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Self::Decode(error) => Some(error),
			Self::FromUtf8(error) => Some(error),
			Self::ParseInt(error) => Some(error),
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

impl From<ParseIntError> for Error {
	fn from(error: ParseIntError) -> Self {
		Self::ParseInt(error)
	}
}

pub type Result<T> = StdResult<T, Error>;
