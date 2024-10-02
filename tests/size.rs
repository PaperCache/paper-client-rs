mod common;

use serial_test::serial;

#[test]
#[serial]
fn size_existent() {
	let mut client = common::init_client(true);

	client.set("key", "value", None).ok();

	let result = client.size("key");
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
