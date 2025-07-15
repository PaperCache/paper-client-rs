mod common;

use paper_client::PaperClient;

const INITIAL_SIZE: u64 = 10 * 1024u64.pow(2);
const UPDATED_SIZE: u64 = 20 * 1024u64.pow(2);

#[test]
fn resize() {
	let mut client = common::init_client(true);

	let result = client.resize(INITIAL_SIZE);
	assert!(result.is_ok());

	let size = get_cache_size(&mut client);
	assert_eq!(size, INITIAL_SIZE);

	let updated = client.resize(UPDATED_SIZE);
	assert!(updated.is_ok());

	let size = get_cache_size(&mut client);
	assert_eq!(size, UPDATED_SIZE);
}

fn get_cache_size(client: &mut PaperClient) -> u64 {
	let status = client
		.status()
		.expect("Could not get cache status.");

	status.max_size()
}
