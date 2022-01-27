use gdapi::Client;

#[tokio::main]
async fn main() {
	let client = Client::new();

	match client.level(128).await {
		Ok(level) => println!("Found level: {} (ID: {})", level.name, level.id),
		Err(error) => println!("Error: {}", error),
	}

	match client.levels(&[11274262, 56568010, 77508963]).await {
		Ok(levels) => println!("Found {} levels!", levels.len()),
		Err(error) => println!("Error: {}", error),
	}
}
