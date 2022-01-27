use gdapi::Client;

#[tokio::main]
async fn main() {
	let mut client = Client::new();

	match client.login("Vaporox", "********").await {
		Ok(login) => println!("Logged in! (ID: {}, Account ID: {})", login.user_id, login.account_id),
		Err(error) => println!("Error: {}", error),
	}

	match client.upload_account_comment("Test comment").await {
		Ok(id) => println!("Uploaded test comment! (ID: {})", id),
		Err(error) => println!("Error: {}", error),
	}
}
