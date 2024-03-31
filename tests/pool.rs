use serial_test::serial;
use paper_client::PaperPool;

#[test]
#[serial]
fn pool_client() {
	let pool = init_pool();

	for _ in 0..10 {
		let result = pool.client().ping();
		assert!(result.is_ok());

		let buf = result.unwrap();
		assert_eq!(buf.to_vec(), b"pong");
	}
}

#[test]
#[serial]
fn pool_auth_invalid() {
	let pool = init_pool();

	let result = pool.client().has("key");
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

fn init_pool() -> PaperPool {
	PaperPool::new("127.0.0.1", 3145, 2)
		.expect("Could not connect pool.")
}
