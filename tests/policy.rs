mod common;

#[cfg(feature = "tokio")]
use paper_client::AsyncPaperClient;
use paper_client::{PaperClient, PaperPolicy};
use serial_test::serial;

const INITIAL_POLICY: PaperPolicy = PaperPolicy::Lfu;
const UPDATED_POLICY: PaperPolicy = PaperPolicy::Fifo;

#[test]
#[serial]
fn policy() {
	let mut client = common::init_client(true);

	let result = client.policy(INITIAL_POLICY);
	assert!(result.is_ok());

	let policy = get_cache_policy(&mut client);
	assert_eq!(policy, INITIAL_POLICY);

	let updated = client.policy(UPDATED_POLICY);
	assert!(updated.is_ok());

	let policy = get_cache_policy(&mut client);
	assert_eq!(policy, UPDATED_POLICY);
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn policy_async() {
	let mut client = common::init_async_client(true).await;

	let result = client.policy(INITIAL_POLICY).await;
	assert!(result.is_ok());

	let policy = get_cache_policy_async(&mut client).await;
	assert_eq!(policy, INITIAL_POLICY);

	let updated = client.policy(UPDATED_POLICY).await;
	assert!(updated.is_ok());

	let policy = get_cache_policy_async(&mut client).await;
	assert_eq!(policy, UPDATED_POLICY);
}

fn get_cache_policy(client: &mut PaperClient) -> PaperPolicy {
	let status = client
		.status()
		.expect("Could not get cache status.");

	*status.policy()
}

async fn get_cache_policy_async(client: &mut AsyncPaperClient) -> PaperPolicy {
	let status = client
		.status()
		.await
		.expect("Could not get cache status.");

	*status.policy()
}
