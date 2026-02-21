mod common;

use serial_test::serial;

#[test]
#[serial]
fn del_existent() {
	let mut client = common::init_client(true);

	assert!(client.set("key", "value", None).is_ok());

	let result = client.del("key");
	assert!(result.is_ok());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn del_existent_async() {
	let mut client = common::init_async_client(true).await;

	assert!(client.set("key", "value", None).await.is_ok());

	let result = client.del("key").await;
	assert!(result.is_ok());
}

#[test]
#[serial]
fn del_non_existent() {
	let mut client = common::init_client(true);

	let result = client.del("key");
	assert!(result.is_err());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn del_non_existent_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.del("key").await;
	assert!(result.is_err());
}
