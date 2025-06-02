use std::error::Error;

use gdapi::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let client = Client::new();

	let map_packs = client.map_packs(6).await?;
	println!("Found {} map packs!", map_packs.len());

	for (i, map_pack) in map_packs.iter().enumerate() {
		println!("Map pack {}: {map_pack:?}", i + 1);
	}

	Ok(())
}
