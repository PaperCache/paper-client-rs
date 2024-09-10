mod common;

use serial_test::serial;

#[test]
#[serial]
fn peek_existent() {
	let mut client = common::init_client(true);

	client.set("key", "value", None).ok();

	let result = client.peek("key");
	assert!(result.is_ok());

	let value: String = result.unwrap()
		.try_into().unwrap();

	assert_eq!(value, "value");
}

#[test]
#[serial]
fn peek_non_existent() {
	let mut client = common::init_client(true);

	let result = client.peek("key");
	assert!(result.is_err());
}
