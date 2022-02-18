use gdapi::client::Client;

#[tokio::main]
async fn main() {
	let mut client = Client::new();

	match client.login("Vaporox", "********").await {
		Ok(user) => println!("Logged in! (ID: {}, Account ID: {})", user.id, user.account_id),
		Err(error) => println!("Error: {}", error),
	}

	match client.upload_account_comment("Test comment").await {
		Ok(id) => println!("Uploaded test comment! (ID: {})", id),
		Err(error) => println!("Error: {}", error),
	}
}
