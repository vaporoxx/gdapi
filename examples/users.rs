use gdapi::client::Client;

#[tokio::main]
async fn main() {
	let client = Client::new();

	match client.search_user("Vaporox").await {
		Ok(user) => println!("Found user: {} (Account ID: {})", user.username, user.account_id),
		Err(error) => println!("Error: {}", error),
	}

	match client.user(71).await {
		Ok(user) => println!("Found user: {} (Account ID: {})", user.username, user.account_id),
		Err(error) => println!("Error: {}", error),
	}
}
