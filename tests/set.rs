mod common;

use std::{thread, time::Duration};

use serial_test::serial;

#[test]
#[serial]
fn set_no_ttl() {
	let mut client = common::init_client(true);

	let result = client.set("key", "value", None);
	assert!(result.is_ok());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn set_no_ttl_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.set("key", "value", None).await;
	assert!(result.is_ok());
}

#[test]
#[serial]
fn set_ttl() {
	let mut client = common::init_client(true);

	let result = client.set("key", "value", Some(1));
	assert!(result.is_ok());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn set_ttl_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.set("key", "value", Some(1)).await;
	assert!(result.is_ok());
}

#[test]
#[serial]
fn set_ttl_expiry() {
	let mut client = common::init_client(true);

	let result = client.set("key", "value", Some(1));
	assert!(result.is_ok());

	let got = client.get("key");
	assert!(got.is_ok());

	thread::sleep(Duration::from_secs(2));

	let expired = client.get("key");
	assert!(expired.is_err());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn set_ttl_expiry_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.set("key", "value", Some(1)).await;
	assert!(result.is_ok());

	let got = client.get("key").await;
	assert!(got.is_ok());

	tokio::time::sleep(Duration::from_secs(2)).await;

	let expired = client.get("key").await;
	assert!(expired.is_err());
}
