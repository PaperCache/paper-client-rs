mod common;

use paper_client::{PaperClient, PaperPolicy};

const INITIAL_POLICY: PaperPolicy = PaperPolicy::Lfu;
const UPDATED_POLICY: PaperPolicy = PaperPolicy::Fifo;

#[test]
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

fn get_cache_policy(client: &mut PaperClient) -> PaperPolicy {
	let status = client
		.status()
		.expect("Could not get cache status.");

	*status.policy()
}
