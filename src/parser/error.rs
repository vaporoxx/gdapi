use std::num::ParseIntError;
use std::{error, fmt};

/// The error type for all parsing actions.
#[derive(Debug)]
pub enum Error {
	/// An enum value is out of range
	InvalidEnumValue(u8),
	/// A required key is not present
	InvalidKey(u8),
	/// An array string has an invalid length
	InvalidLength,
	/// A key-value string has an odd length
	OddElements,
	/// A wrapper around a [`ParseIntError`]
	ParseInt(ParseIntError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidEnumValue(key) => write!(f, "enum value out of range (key {key})"),
			Self::InvalidKey(key) => write!(f, "required key not present (key {key})"),
			Self::InvalidLength => write!(f, "array string has invalid length"),
			Self::OddElements => write!(f, "key-value string has odd length"),
			Self::ParseInt(error) => error.fmt(f),
		}
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Self::InvalidEnumValue(_) => None,
			Self::InvalidKey(_) => None,
			Self::InvalidLength => None,
			Self::OddElements => None,
			Self::ParseInt(error) => Some(error),
		}
	}
}

impl From<ParseIntError> for Error {
	fn from(error: ParseIntError) -> Self {
		Self::ParseInt(error)
	}
}
