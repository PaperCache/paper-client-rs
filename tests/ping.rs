mod common;

#[test]
fn ping() {
	let mut client = common::init_client(true);

	let result = client.ping();
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.to_vec(), b"pong");
}
