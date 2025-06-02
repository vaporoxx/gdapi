use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::{FromStr, Split};

use crate::crypto::decode;
use crate::error::Result;
use crate::parser::error::Error;
use crate::parser::parse::Parse;

pub struct List<'a> {
	inner: Split<'a, char>,
}

impl<'a> List<'a> {
	pub fn new(data: &'a str, sep: char) -> Self {
		Self { inner: data.split(sep) }
	}

	pub fn ints<T: FromStr<Err = ParseIntError>, const N: usize>(self) -> Result<[T; N]> {
		self.inner
			.map(|value| value.parse().map_err(Error::from))
			.collect::<Result<Vec<_>, _>>()?
			.try_into()
			.or(Err(Error::InvalidLength.into()))
	}

	pub fn next(&mut self) -> Result<&'a str> {
		Ok(self.inner.next().ok_or(Error::InvalidLength)?)
	}

	pub fn strs<const N: usize>(self) -> Result<[&'a str; N]> {
		self.inner
			.collect::<Vec<_>>()
			.try_into()
			.or(Err(Error::InvalidLength.into()))
	}

	pub fn vec<T: Parse>(self) -> Result<Vec<T>> {
		self.inner.map(|value| T::parse(value, None)).collect()
	}
}

pub struct Map<'a> {
	inner: HashMap<u8, &'a str>,
}

impl<'a> Map<'a> {
	pub fn new(data: &'a str, sep: char) -> Result<Self> {
		let mut inner = HashMap::new();
		let mut split = data.split(sep);

		while let Some(next) = split.next() {
			let key = next.parse().map_err(Error::from)?;
			let value = split.next().ok_or(Error::OddElements)?;

			inner.insert(key, value);
		}

		Ok(Self { inner })
	}

	pub fn base64(&self, key: u8) -> Result<String> {
		decode::base64(self.str(key)?)
	}

	pub fn bool(&self, key: u8) -> Result<bool> {
		Ok(!self.str(key)?.is_empty())
	}

	pub fn int<T: FromStr<Err = ParseIntError>>(&self, key: u8) -> Result<T> {
		Ok(self.str(key)?.parse().map_err(Error::from)?)
	}

	pub fn list(&self, key: u8, sep: char) -> Result<List<'a>> {
		Ok(List::new(self.str(key)?, sep))
	}

	pub fn str(&self, key: u8) -> Result<&'a str> {
		Ok(self.inner.get(&key).copied().ok_or(Error::InvalidKey(key))?)
	}

	pub fn string(&self, key: u8) -> Result<String> {
		Ok(self.str(key)?.into())
	}
}
