mod common;

use serial_test::serial;

#[test]
#[serial]
fn peek_existent() {
	let mut client = common::init_client(true);

	let value = b"value"
		.to_vec()
		.into_boxed_slice();

	client.set("key", &value, None).ok();

	let result = client.peek("key");
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf, value);
}

#[test]
#[serial]
fn peek_non_existent() {
	let mut client = common::init_client(true);

	let result = client.peek("key");
	assert!(result.is_err());
}
