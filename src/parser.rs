use std::collections::HashMap;

pub type APIData = HashMap<u8, String>;

pub fn parse_data(raw: &str) -> Option<APIData> {
	let split: Vec<_> = raw.split(':').collect();
	let mut data = HashMap::new();

	for chunk in split.chunks(2) {
		data.insert(chunk[0].parse().ok()?, chunk[1].into());
	}

	Some(data)
}
