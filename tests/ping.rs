mod common;

#[test]
fn ping() {
	let mut client = common::init_client(true);

	let result = client.ping();
	assert!(result.is_ok());

	let value: String = result.unwrap().try_into().unwrap();

	assert_eq!(value, "pong");
}
