mod common;

#[test]
fn ping() {
	let mut client = common::init_client(false);

	let result = client.ping();
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();
	assert_eq!(value, "pong");
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn ping_async() {
	let mut client = common::init_async_client(false).await;

	let result = client.ping().await;
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();
	assert_eq!(value, "pong");
}
