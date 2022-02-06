use super::structs::*;
use std::{collections::HashMap, hash::Hash};

/// Data structures that are identifiable by a unique identifier.
pub trait Identifiable {
	/// The type of the identifier
	type Id: Eq + Hash;

	/// Retrieves the identifier for this structure.
	fn id(&self) -> Self::Id;
}

impl Identifiable for Gauntlet {
	type Id = u8;

	fn id(&self) -> Self::Id {
		self.id
	}
}

impl Identifiable for Level {
	type Id = u32;

	fn id(&self) -> Self::Id {
		self.id
	}
}

impl Identifiable for MapPack {
	type Id = u8;

	fn id(&self) -> Self::Id {
		self.id
	}
}

impl Identifiable for User {
	type Id = u32;

	fn id(&self) -> Self::Id {
		self.id
	}
}

/// Convenience type for a hash map of [`Identifiable`] values.
pub type Map<T> = HashMap<<T as Identifiable>::Id, T>;
