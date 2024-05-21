mod common;

use paper_client::FromPaperValue;

#[test]
fn auth_incorrect() {
	let mut client = common::init_client(false);

	let result = client.auth("incorrect_auth_token");
	assert!(result.is_err());
}

#[test]
fn auth_correct() {
	let mut client = common::init_client(false);

	let result = client.auth("auth_token");
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.into_string(), "done");
}
