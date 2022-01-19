use gdapi::Client;

#[tokio::main]
async fn main() {
	let mut client = Client::new();

	match client.gauntlets().await {
		Ok(gauntlets) => println!("Found {} gauntlets!", gauntlets.len()),
		Err(error) => println!("Error: {}", error),
	}

	match client.level(128).await {
		Ok(level) => println!("Found level: {} (ID: {})", level.name, level.id),
		Err(error) => println!("Error: {}", error),
	}

	match client.login("Vaporox", "********").await {
		Ok(login) => println!("Logged in! (ID: {}, Account ID: {})", login.user_id, login.account_id),
		Err(error) => println!("Error: {}", error),
	}

	match client.map_packs().page(6).await {
		Ok(packs) => println!("Found {} map packs!", packs.len()),
		Err(error) => println!("Error: {}", error),
	}

	match client.search_user("Vaporox").await {
		Ok(user) => println!("Found user: {} (Account ID: {})", user.username, user.account_id),
		Err(error) => println!("Error: {}", error),
	}

	match client.user(71).await {
		Ok(user) => println!("Found user: {} (Account ID: {})", user.username, user.account_id),
		Err(error) => println!("Error: {}", error),
	}
}
