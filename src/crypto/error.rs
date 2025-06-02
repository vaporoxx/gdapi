use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::{error, fmt};

use base64::DecodeError;

/// The error type for all cryptographic actions.
#[derive(Debug)]
pub enum Error {
	/// A wrapper around a [`DecodeError`]
	Decode(DecodeError),
	/// A wrapper around a [`FromUtf8Error`]
	FromUtf8(FromUtf8Error),
	/// A wrapper around a [`ParseIntError`]
	ParseInt(ParseIntError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Decode(error) => error.fmt(f),
			Self::FromUtf8(error) => error.fmt(f),
			Self::ParseInt(error) => error.fmt(f),
		}
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
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
