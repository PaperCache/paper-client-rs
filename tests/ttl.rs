mod common;

use serial_test::serial;

#[test]
#[serial]
fn ttl_existent() {
	let mut client = common::init_client(true);

	client.set("key", "value", None).ok();

	let result = client.ttl("key", Some(1));
	assert!(result.is_ok());

	let value: String = result.unwrap()
		.try_into().unwrap();

	assert_eq!(value, "done");
}

#[test]
#[serial]
fn ttl_non_existent() {
	let mut client = common::init_client(true);

	let result = client.ttl("key", Some(1));
	assert!(result.is_err());
}
