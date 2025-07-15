mod common;

#[test]
fn status() {
	let mut client = common::init_client(true);

	let result = client.status();
	assert!(result.is_ok());
}
