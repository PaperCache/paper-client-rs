mod common;

use serial_test::serial;

#[test]
#[serial]
fn has_existent() {
	let mut client = common::init_client(true);

	assert!(client.set("key", "value", None).is_ok());

	let result = client.has("key");
	assert!(result.is_ok());

	let has = result.unwrap();
	assert!(has);
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn has_existent_async() {
	let mut client = common::init_async_client(true).await;

	assert!(client.set("key", "value", None).await.is_ok());

	let result = client.has("key").await;
	assert!(result.is_ok());

	let has = result.unwrap();
	assert!(has);
}

#[test]
#[serial]
fn has_non_existent() {
	let mut client = common::init_client(true);

	let result = client.has("key");
	assert!(result.is_ok());

	let has = result.unwrap();
	assert!(!has);
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn has_non_existent_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.has("key").await;
	assert!(result.is_ok());

	let has = result.unwrap();
	assert!(!has);
}
