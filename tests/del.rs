mod common;

use serial_test::serial;

#[test]
#[serial]
fn del_existent() {
	let mut client = common::init_client(true);

	client.set("key", "value", None).ok();

	let result = client.del("key");
	assert!(result.is_ok());

	let value: String = result.unwrap()
		.try_into().unwrap();

	assert_eq!(value, "done");
}

#[test]
#[serial]
fn del_non_existent() {
	let mut client = common::init_client(true);

	let result = client.del("key");
	assert!(result.is_err());
}
