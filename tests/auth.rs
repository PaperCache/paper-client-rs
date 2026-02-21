mod common;

#[test]
fn auth_incorrect() {
	let mut client = common::init_client(false);

	let result = client.auth("incorrect_auth_token");
	assert!(result.is_err());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn auth_incorrect_async() {
	let mut client = common::init_async_client(false).await;

	let result = client.auth("incorrect_auth_token").await;
	assert!(result.is_err());
}

#[test]
fn auth_correct() {
	let mut client = common::init_client(false);

	let result = client.auth("auth_token");
	assert!(result.is_ok());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn auth_correct_async() {
	let mut client = common::init_async_client(false).await;

	let result = client.auth("auth_token").await;
	assert!(result.is_ok());
}
