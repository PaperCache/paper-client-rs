#[cfg(feature = "tokio")]
use paper_client::AsyncPaperPool;
use paper_client::PaperPool;
use serial_test::serial;

#[test]
#[serial]
fn pool_client() {
	let pool = init_pool();

	for _ in 0..10 {
		let result = pool.client().ping();
		assert!(result.is_ok());

		let value: String = result.unwrap().try_into().unwrap();
		assert_eq!(value, "pong");
	}
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn pool_client_async() {
	let pool = init_async_pool().await;

	for _ in 0..10 {
		let result = pool.client().await.ping().await;
		assert!(result.is_ok());

		let value: String = result.unwrap().try_into().unwrap();
		assert_eq!(value, "pong");
	}
}

#[test]
#[serial]
fn pool_auth_invalid() {
	let pool = init_pool();

	let result = pool.client().has("key");
	assert!(result.is_err());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn pool_auth_invalid_async() {
	let pool = init_async_pool().await;

	let result = pool.client().await.has("key").await;
	assert!(result.is_err());
}

#[test]
#[serial]
fn pool_auth_valid() {
	let pool = init_pool();

	pool.auth("auth_token")
		.expect("Could not authorize pool.");

	let result = pool.client().has("key");
	assert!(result.is_ok());
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn pool_auth_valid_async() {
	let pool = init_async_pool().await;

	pool.auth("auth_token")
		.await
		.expect("Could not authorize pool.");

	let result = pool.client().await.has("key").await;
	assert!(result.is_ok());
}

fn init_pool() -> PaperPool {
	PaperPool::new("paper://127.0.0.1:3145", 2).expect("Could not connect pool.")
}

async fn init_async_pool() -> AsyncPaperPool {
	AsyncPaperPool::new("paper://127.0.0.1:3145", 2)
		.await
		.expect("Could not connect pool.")
}
