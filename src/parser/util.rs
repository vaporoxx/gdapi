use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::{FromStr, Split};

use crate::crypto::decode;
use crate::error::{Error, ParseError, Result};
use crate::parser::parse::Parse;

pub struct List<'a> {
	inner: Split<'a, char>,
}

impl<'a> List<'a> {
	pub fn ints<T: FromStr<Err = ParseIntError>, const N: usize>(self) -> Result<[T; N]> {
		self.inner
			.map(|value| value.parse().map_err(Error::from))
			.collect::<Result<Vec<_>>>()?
			.try_into()
			.map_err(|_| ParseError::InvalidLength.into())
	}

	pub fn new(data: &'a str, sep: char) -> Self {
		Self { inner: data.split(sep) }
	}

	pub fn next(&mut self) -> Result<&'a str> {
		self.inner.next().ok_or_else(|| ParseError::InvalidLength.into())
	}

	pub fn strs<const N: usize>(self) -> Result<[&'a str; N]> {
		self.inner
			.collect::<Vec<_>>()
			.try_into()
			.map_err(|_| ParseError::InvalidLength.into())
	}

	pub fn vec<T: Parse>(self) -> Result<Vec<T>> {
		self.inner.map(|value| T::parse(value, None)).collect()
	}
}

pub struct Map<'a> {
	inner: HashMap<u8, &'a str>,
}

impl<'a> Map<'a> {
	pub fn base64(&self, key: u8) -> Result<String> {
		self.str(key).and_then(decode::base64)
	}

	pub fn bool(&self, key: u8) -> Result<bool> {
		self.str(key).map(|data| !data.is_empty())
	}

	pub fn int<T: FromStr<Err = ParseIntError>>(&self, key: u8) -> Result<T> {
		self.str(key).and_then(|data| data.parse().map_err(Error::from))
	}

	pub fn list(&self, key: u8, sep: char) -> Result<List<'a>> {
		self.str(key).map(|data| List::new(data, sep))
	}

	pub fn new(data: &'a str, sep: char) -> Result<Self> {
		let mut inner = HashMap::new();
		let mut split = data.split(sep);

		while let Some(next) = split.next() {
			let key = next.parse()?;
			let value = split.next().ok_or(ParseError::OddElements)?;

			inner.insert(key, value);
		}

		Ok(Self { inner })
	}

	pub fn str(&self, key: u8) -> Result<&'a str> {
		self.inner
			.get(&key)
			.copied()
			.ok_or_else(|| ParseError::InvalidKey(key).into())
	}

	pub fn string(&self, key: u8) -> Result<String> {
		self.str(key).map(String::from)
	}
}
