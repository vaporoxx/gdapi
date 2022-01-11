use gdapi::Client;

#[tokio::main]
async fn main() {
	let client = Client::new();

	match client.level(128).await {
		Ok(level) => println!("Found level: {} (ID: {})", level.name, level.id),
		Err(error) => println!("Error: {}", error),
	}

	match client.user(71).await {
		Ok(user) => println!("Found user: {} (ID: {})", user.name, user.id),
		Err(error) => println!("Error: {}", error),
	}
}
