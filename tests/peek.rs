mod common;

use serial_test::serial;
use paper_client::FromPaperValue;

#[test]
#[serial]
fn peek_existent() {
	let mut client = common::init_client(true);

	client.set("key", "value", None).ok();

	let result = client.peek("key");
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.into_string(), "value");
}

#[test]
#[serial]
fn peek_non_existent() {
	let mut client = common::init_client(true);

	let result = client.peek("key");
	assert!(result.is_err());
}
