mod common;

#[test]
fn stats() {
	let mut client = common::init_client(true);

	let result = client.stats();
	assert!(result.is_ok());
}
