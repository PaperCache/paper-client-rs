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

#[cfg(feature = "tokio")]
#[tokio::test]
async fn wipe_async() {
	let mut client = common::init_async_client(true).await;

	let set = client.set("key", "value", Some(1)).await;
	assert!(set.is_ok());

	let result = client.wipe().await;
	assert!(result.is_ok());

	let got = client.get("key").await;
	assert!(got.is_err());
}
