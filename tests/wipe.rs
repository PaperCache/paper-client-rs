mod common;

use paper_client::FromPaperValue;

#[test]
fn wipe() {
	let mut client = common::init_client(true);

	let set = client.set("key", "value", Some(1));
	assert!(set.is_ok());

	let result = client.wipe();
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.into_string(), "done");

	let got = client.get("key");
	assert!(got.is_err());
}
