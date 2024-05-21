mod common;

use serial_test::serial;
use paper_client::FromPaperValue;

#[test]
#[serial]
fn ttl_existent() {
	let mut client = common::init_client(true);

	client.set("key", "value", None).ok();

	let result = client.ttl("key", Some(1));
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.into_string(), "done");
}

#[test]
#[serial]
fn ttl_non_existent() {
	let mut client = common::init_client(true);

	let result = client.ttl("key", Some(1));
	assert!(result.is_err());
}
