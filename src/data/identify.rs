use super::structs::*;
use std::{collections::HashMap, hash::Hash};

/// Data structures that are identifiable by a unique identifier.
pub trait Identify {
	/// The type of the identifier
	type Id: Eq + Hash;

	/// Retrieves the identifier for this structure.
	fn id(&self) -> Self::Id;
}

impl Identify for Gauntlet {
	type Id = u8;

	fn id(&self) -> Self::Id {
		self.id
	}
}

impl Identify for Level {
	type Id = u32;

	fn id(&self) -> Self::Id {
		self.id
	}
}

impl Identify for LoginUser {
	type Id = u32;

	fn id(&self) -> Self::Id {
		self.id
	}
}

impl Identify for MapPack {
	type Id = u8;

	fn id(&self) -> Self::Id {
		self.id
	}
}

impl Identify for User {
	type Id = u32;

	fn id(&self) -> Self::Id {
		self.id
	}
}

/// Convenience type for a hash map of [`Identify`] values.
pub type Map<T> = HashMap<<T as Identify>::Id, T>;
