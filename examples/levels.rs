use std::error::Error;

use gdapi::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let client = Client::new();

	let level = client.level(128).await?;
	println!("Found level: {:?}", level);

	let levels = client.levels(&[11274262, 56568010, 77508963]).await?;
	println!("Found {} levels!", levels.len());

	for (i, level) in levels.iter().enumerate() {
		println!("Level {}: {:?}", i + 1, level);
	}

	Ok(())
}
