mod common;

use serial_test::serial;

#[test]
#[serial]
fn size_existent() {
	let mut client = common::init_client(true);

	let value = b"value"
		.to_vec()
		.into_boxed_slice();

	client.set("key", &value, None).ok();

	let result = client.size("key");
	assert!(result.is_ok());

	let size = result.unwrap();
	assert_eq!(size, value.len() as u64);
}

#[test]
#[serial]
fn size_non_existent() {
	let mut client = common::init_client(true);

	let result = client.size("key");
	assert!(result.is_err());
}
