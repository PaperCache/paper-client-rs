mod common;

use serial_test::serial;

#[test]
#[serial]
fn ttl_existent() {
	let mut client = common::init_client(true);

	assert!(client.set("key", "value", None).is_ok());

	let result = client.ttl("key", Some(1));
	assert!(result.is_ok());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn ttl_existent_async() {
	let mut client = common::init_async_client(true).await;

	assert!(client.set("key", "value", None).await.is_ok());

	let result = client.ttl("key", Some(1)).await;
	assert!(result.is_ok());
}

#[test]
#[serial]
fn ttl_non_existent() {
	let mut client = common::init_client(true);

	let result = client.ttl("key", Some(1));
	assert!(result.is_err());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn ttl_non_existent_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.ttl("key", Some(1)).await;
	assert!(result.is_err());
}
