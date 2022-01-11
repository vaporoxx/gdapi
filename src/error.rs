use reqwest::Error as ReqwestError;
use std::{error::Error as StdError, fmt::Display, result::Result as StdResult};

#[derive(Debug)]
pub enum Error {
	InvalidRequest,
	ParseResponse,
	Reqwest(ReqwestError),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidRequest => "server received an invalid request".fmt(f),
			Self::ParseResponse => "server response could not be parsed".fmt(f),
			Self::Reqwest(error) => error.fmt(f),
		}
	}
}

impl StdError for Error {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Self::InvalidRequest => None,
			Self::ParseResponse => None,
			Self::Reqwest(error) => Some(error),
		}
	}
}

impl From<ReqwestError> for Error {
	fn from(error: ReqwestError) -> Self {
		Self::Reqwest(error)
	}
}

pub type Result<T> = StdResult<T, Error>;
