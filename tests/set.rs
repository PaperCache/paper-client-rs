mod common;

use std::{
	thread,
	time::Duration,
};

use serial_test::serial;

#[test]
#[serial]
fn set_no_ttl() {
	let mut client = common::init_client(true);

	let value = b"value"
		.to_vec()
		.into_boxed_slice();

	let result = client.set("key", &value, None);
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.to_vec(), b"done");
}

#[test]
#[serial]
fn set_ttl() {
	let mut client = common::init_client(true);

	let value = b"value"
		.to_vec()
		.into_boxed_slice();

	let result = client.set("key", &value, Some(1));
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.to_vec(), b"done");
}

#[test]
#[serial]
fn set_ttl_expiry() {
	let mut client = common::init_client(true);

	let value = b"value"
		.to_vec()
		.into_boxed_slice();

	let result = client.set("key", &value, Some(1));
	assert!(result.is_ok());

	let got = client.get("key");
	assert!(got.is_ok());

	thread::sleep(Duration::from_secs(2));

	let expired = client.get("key");
	assert!(expired.is_err());
}
