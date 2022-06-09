use std::error::Error;

use gdapi::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let client = Client::new();

	let levels = client.gauntlet(10).await?;
	println!("The gauntlet contains {} levels!", levels.len());

	for (i, level) in levels.iter().enumerate() {
		println!("Level {}: {:?}", i + 1, level);
	}

	let gauntlets = client.gauntlets().await?;
	println!("Found {} gauntlets!", gauntlets.len());

	for (i, gauntlet) in gauntlets.iter().enumerate() {
		println!("Gauntlet {}: {:?}", i + 1, gauntlet);
	}

	Ok(())
}
