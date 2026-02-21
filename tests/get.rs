mod common;

use serial_test::serial;

#[test]
#[serial]
fn get_existent() {
	let mut client = common::init_client(true);

	assert!(client.set("key", "value", None).is_ok());

	let result = client.get("key");
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();
	assert_eq!(value, "value");
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn get_existent_async() {
	let mut client = common::init_async_client(true).await;

	assert!(client.set("key", "value", None).await.is_ok());

	let result = client.get("key").await;
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();
	assert_eq!(value, "value");
}

#[test]
#[serial]
fn get_non_existent() {
	let mut client = common::init_client(true);

	let result = client.get("key");
	assert!(result.is_err());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn get_non_existent_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.get("key").await;
	assert!(result.is_err());
}
