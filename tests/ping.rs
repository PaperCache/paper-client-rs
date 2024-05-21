mod common;

use paper_client::FromPaperValue;

#[test]
fn ping() {
	let mut client = common::init_client(true);

	let result = client.ping();
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.into_string(), "pong");
}
