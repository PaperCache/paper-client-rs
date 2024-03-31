mod common;

#[test]
fn version() {
	let mut client = common::init_client(true);

	let result = client.version();
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert!(!buf.is_empty());
}
