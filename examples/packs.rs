use gdapi::client::Client;

#[tokio::main]
async fn main() {
	let client = Client::new();

	match client.gauntlet(10).await {
		Ok(levels) => println!("The gauntlet contains {} levels!", levels.len()),
		Err(error) => println!("Error: {}", error),
	}

	match client.gauntlets().await {
		Ok(gauntlets) => println!("Found {} gauntlets!", gauntlets.len()),
		Err(error) => println!("Error: {}", error),
	}

	match client.map_packs(6).await {
		Ok(packs) => println!("Found {} map packs!", packs.len()),
		Err(error) => println!("Error: {}", error),
	}
}
