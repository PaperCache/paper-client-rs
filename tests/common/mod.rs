use paper_client::PaperClient;

pub fn init_client(authed: bool) -> PaperClient {
	let mut client = PaperClient::new("paper://127.0.0.1:3145")
		.expect("Could not initialize client.");

	if authed {
		client.auth("auth_token")
			.expect("Could not authorize client.");

		client.wipe()
			.expect("Could not wipe client.");
	}

	client
}
