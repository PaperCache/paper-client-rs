mod common;

#[test]
fn version() {
	let mut client = common::init_client(true);

	let result = client.version();
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();

	assert!(!value.is_empty());
}
