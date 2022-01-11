use reqwest::Error as ReqwestError;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum Error {
	InvalidRequest,
	ParseResponse,
	Reqwest(ReqwestError),
}

impl From<ReqwestError> for Error {
	fn from(error: ReqwestError) -> Self {
		Self::Reqwest(error)
	}
}

pub type Result<T> = StdResult<T, Error>;
