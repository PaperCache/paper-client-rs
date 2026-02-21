mod common;

#[test]
fn status() {
	let mut client = common::init_client(true);

	let result = client.status();
	assert!(result.is_ok());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn status_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.status().await;
	assert!(result.is_ok());
}
