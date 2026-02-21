#[cfg(feature = "tokio")]
use paper_client::AsyncPaperClient;
use paper_client::PaperClient;

pub fn init_client(authed: bool) -> PaperClient {
	let mut client =
		PaperClient::new("paper://127.0.0.1:3145").expect("Could not initialize client.");

	if authed {
		client
			.auth("auth_token")
			.expect("Could not authorize client.");

		client.wipe().expect("Could not wipe client.");
	}

	client
}

#[cfg(feature = "tokio")]
pub async fn init_async_client(authed: bool) -> AsyncPaperClient {
	let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145")
		.await
		.expect("Could not initialize client.");

	if authed {
		client
			.auth("auth_token")
			.await
			.expect("Could not authorize client.");

		client
			.wipe()
			.await
			.expect("Could not wipe client.");
	}

	client
}
