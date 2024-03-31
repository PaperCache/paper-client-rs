mod common;

use serial_test::serial;

#[test]
#[serial]
fn has_existent() {
	let mut client = common::init_client(true);

	let value = b"value"
		.to_vec()
		.into_boxed_slice();

	client.set("key", &value, None).ok();

	let result = client.has("key");
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
