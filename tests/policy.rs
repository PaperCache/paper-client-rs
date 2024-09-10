mod common;

use paper_client::{PaperClient, Policy};

const INITIAL_POLICY: Policy = Policy::Lfu;
const UPDATED_POLICY: Policy = Policy::Fifo;

#[test]
fn policy() {
	let mut client = common::init_client(true);

	let result = client.policy(INITIAL_POLICY);
	assert!(result.is_ok());

	let value: String = result.unwrap()
		.try_into().unwrap();

	assert_eq!(value, "done");

	let policy = get_cache_policy(&mut client);
	assert_eq!(policy, INITIAL_POLICY);

	let updated = client.policy(UPDATED_POLICY);
	assert!(updated.is_ok());

	let value: String = updated.unwrap()
		.try_into().unwrap();

	assert_eq!(value, "done");

	let policy = get_cache_policy(&mut client);
	assert_eq!(policy, UPDATED_POLICY);
}

fn get_cache_policy(client: &mut PaperClient) -> Policy {
	let stats = client
		.stats()
		.expect("Could not get cache size.");

	*stats.get_policy()
}
