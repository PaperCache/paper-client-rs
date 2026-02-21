mod common;

#[test]
fn version() {
	let mut client = common::init_client(false);

	let result = client.version();
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();
	assert!(!value.is_empty());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn version_async() {
	let mut client = common::init_async_client(false).await;

	let result = client.version().await;
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();
	assert!(!value.is_empty());
}
