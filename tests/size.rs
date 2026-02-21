mod common;

use serial_test::serial;

#[test]
#[serial]
fn size_existent() {
	let mut client = common::init_client(true);

	assert!(client.set("key", "value", None).is_ok());

	let result = client.size("key");
	assert!(result.is_ok());

	let size = result.unwrap();
	assert!(size > 0);
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn size_existent_async() {
	let mut client = common::init_async_client(true).await;

	assert!(client.set("key", "value", None).await.is_ok());

	let result = client.size("key").await;
	assert!(result.is_ok());

	let size = result.unwrap();
	assert!(size > 0);
}

#[test]
#[serial]
fn size_non_existent() {
	let mut client = common::init_client(true);

	let result = client.size("key");
	assert!(result.is_err());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn size_non_existent_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.size("key").await;
	assert!(result.is_err());
}
