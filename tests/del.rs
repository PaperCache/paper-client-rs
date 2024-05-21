mod common;

use serial_test::serial;
use paper_client::FromPaperValue;

#[test]
#[serial]
fn del_existent() {
	let mut client = common::init_client(true);

	client.set("key", "value", None).ok();

	let result = client.del("key");
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.into_string().unwrap(), "done");
}

#[test]
#[serial]
fn del_non_existent() {
	let mut client = common::init_client(true);

	let result = client.del("key");
	assert!(result.is_err());
}
