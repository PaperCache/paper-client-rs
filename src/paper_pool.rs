use std::sync::{
	Arc,
	Mutex,
	MutexGuard,
	atomic::{Ordering, AtomicUsize},
};

use crate::paper_client::{PaperClient, PaperClientError};

#[derive(Clone)]
pub struct PaperPool {
	clients: Arc<Box<[Arc<Mutex<PaperClient>>]>>,
	index: Arc<AtomicUsize>,
}

impl PaperPool {
	/// Creates a new instance of a pool of clients of size `size`.
	/// If a connection could not be established to any of the clients,
	/// a `PaperClientError` is returned.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperPool;
	///
	/// let pool = PaperPool::new("127.0.0.1", 3145, 4).unwrap();
	/// ```
	pub fn new(
		host: &str,
		port: u32,
		size: usize,
	) -> Result<Self, PaperClientError> {
		assert!(size > 0);

		let mut clients = Vec::new();

		for _ in 0..size {
			let client = PaperClient::new(host, port)?;
			clients.push(Arc::new(Mutex::new(client)));
		}

		let pool = PaperPool {
			clients: Arc::new(clients.into_boxed_slice()),
			index: Arc::new(AtomicUsize::default()),
		};

		Ok(pool)
	}

	/// Obtains a guarded `PaperClient`. Use this client, then drop the
	/// reference (or allow it to go out of scope). Do not hold a reference
	/// to this client, otherwise the client will be unusable by other
	/// threads in the future.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperPool;
	///
	/// let pool = PaperPool::new("127.0.0.1", 3145, 4).unwrap();
	///
	/// match pool.client().ping() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// };
	/// ```
	pub fn client(&self) -> MutexGuard<PaperClient> {
		self.clients[self.get_index()]
			.lock().expect("Could not obtain client.")
	}

	fn get_index(&self) -> usize {
		let index = self.index.load(Ordering::Relaxed);
		self.index.store((index + 1) % self.clients.len(), Ordering::Relaxed);
		index
	}
}
