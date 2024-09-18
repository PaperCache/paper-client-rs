mod common;

#[test]
fn wipe() {
	let mut client = common::init_client(true);

	let set = client.set("key", "value", Some(1));
	assert!(set.is_ok());

	let result = client.wipe();
	assert!(result.is_ok());

	let got = client.get("key");
	assert!(got.is_err());
}
