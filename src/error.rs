//! The error type and all related types.

use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::{error, fmt, result};

use base64::DecodeError;
use reqwest::Error as ReqwestError;

/// The error type used across the library.
#[derive(Debug)]
pub enum Error {
	/// A wrapper around a [`DecodeError`].
	Decode(DecodeError),
	/// A wrapper around a [`FromUtf8Error`].
	FromUtf8(FromUtf8Error),
	/// A wrapper around a [`ParseError`].
	Parse(ParseError),
	/// A wrapper around a [`ParseIntError`].
	ParseInt(ParseIntError),
	/// A wrapper around a [`RequestError`].
	Request(RequestError),
	/// A wrapper around a [`ReqwestError`].
	Reqwest(ReqwestError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Decode(error) => error.fmt(f),
			Self::FromUtf8(error) => error.fmt(f),
			Self::Parse(error) => error.fmt(f),
			Self::ParseInt(error) => error.fmt(f),
			Self::Request(error) => error.fmt(f),
			Self::Reqwest(error) => error.fmt(f),
		}
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Self::Decode(error) => Some(error),
			Self::FromUtf8(error) => Some(error),
			Self::Parse(error) => Some(error),
			Self::ParseInt(error) => Some(error),
			Self::Request(error) => Some(error),
			Self::Reqwest(error) => Some(error),
		}
	}
}

impl From<DecodeError> for Error {
	fn from(value: DecodeError) -> Self {
		Self::Decode(value)
	}
}

impl From<FromUtf8Error> for Error {
	fn from(value: FromUtf8Error) -> Self {
		Self::FromUtf8(value)
	}
}

impl From<ParseError> for Error {
	fn from(value: ParseError) -> Self {
		Self::Parse(value)
	}
}

impl From<ParseIntError> for Error {
	fn from(value: ParseIntError) -> Self {
		Self::ParseInt(value)
	}
}

impl From<RequestError> for Error {
	fn from(value: RequestError) -> Self {
		Self::Request(value)
	}
}

impl From<ReqwestError> for Error {
	fn from(value: ReqwestError) -> Self {
		Self::Reqwest(value)
	}
}

/// The error type for all parse actions.
#[derive(Debug)]
pub enum ParseError {
	/// An enum value is out of range.
	InvalidEnumValue(u8),
	/// A required key is not present.
	InvalidKey(u8),
	/// An array string has an invalid length.
	InvalidLength,
	/// A key-value string has an odd length.
	OddElements,
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidEnumValue(key) => write!(f, "enum value out of range (key {key})"),
			Self::InvalidKey(key) => write!(f, "required key not present (key {key})"),
			Self::InvalidLength => write!(f, "array string has invalid length"),
			Self::OddElements => write!(f, "key-value string has odd length"),
		}
	}
}

impl error::Error for ParseError {}

/// The error type for all request actions.
#[derive(Debug)]
pub enum RequestError {
	/// The client is already logged in.
	AlreadyLoggedIn,
	/// The server rejected the request.
	InvalidRequest(i8),
	/// The client needs to be logged in.
	NotLoggedIn,
}

impl fmt::Display for RequestError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::AlreadyLoggedIn => write!(f, "client is already logged in"),
			Self::InvalidRequest(code) => write!(f, "server rejected the request (status code {code})"),
			Self::NotLoggedIn => write!(f, "client needs to be logged in"),
		}
	}
}

impl error::Error for RequestError {}

/// The standard result type, but with the error set to [`Error`].
pub type Result<T> = result::Result<T, Error>;
