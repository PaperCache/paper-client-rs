mod common;

use paper_client::{PaperClient, FromPaperValue};

const INITIAL_SIZE: u64 = 10 * 1024u64.pow(2);
const UPDATED_SIZE: u64 = 20 * 1024u64.pow(2);

#[test]
fn resize() {
	let mut client = common::init_client(true);

	let result = client.resize(INITIAL_SIZE);
	assert!(result.is_ok());

	let buf = result.unwrap();
	assert_eq!(buf.into_string().unwrap(), "done");

	let size = get_cache_size(&mut client);
	assert_eq!(size, INITIAL_SIZE);

	let updated = client.resize(UPDATED_SIZE);
	assert!(updated.is_ok());

	let buf = updated.unwrap();
	assert_eq!(buf.into_string().unwrap(), "done");

	let size = get_cache_size(&mut client);
	assert_eq!(size, UPDATED_SIZE);
}

fn get_cache_size(client: &mut PaperClient) -> u64 {
	let stats = client
		.stats()
		.expect("Could not get cache size.");

	stats.get_max_size()
}
