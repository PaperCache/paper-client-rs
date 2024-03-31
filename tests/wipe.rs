mod common;

#[test]
fn wipe() {
	let mut client = common::init_client(true);

	let value = b"value"
		.to_vec()
		.into_boxed_slice();

	let set = client.set("key", &value, Some(1));
	assert!(set.is_ok());

	let result = client.wipe();
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.to_vec(), b"done");

	let got = client.get("key");
	assert!(got.is_err());
}
