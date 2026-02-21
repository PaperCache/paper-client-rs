/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::{
	Arc,
	Mutex,
	MutexGuard,
	atomic::{AtomicUsize, Ordering},
};

use crate::{addr::FromPaperAddr, client::PaperClient, error::PaperClientError};

#[derive(Debug, Clone)]
pub struct PaperPool {
	clients: Arc<Box<[Arc<Mutex<PaperClient>>]>>,
	index:   Arc<AtomicUsize>,
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
	/// let pool = PaperPool::new("paper://127.0.0.1:3145", 4).unwrap();
	/// ```
	pub fn new(paper_addr: impl FromPaperAddr, size: usize) -> Result<Self, PaperClientError> {
		assert!(size > 0);

		let mut clients = Vec::new();

		for _ in 0..size {
			let client = PaperClient::new(paper_addr.clone())?;
			clients.push(Arc::new(Mutex::new(client)));
		}

		let pool = PaperPool {
			clients: Arc::new(clients.into_boxed_slice()),
			index:   Arc::new(AtomicUsize::default()),
		};

		Ok(pool)
	}

	/// Attempts to authorize each client with the supplied auth token.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperPool;
	///
	/// let pool = PaperPool::new("paper://127.0.0.1:3145", 4).unwrap();
	///
	/// if let Err(err) = pool.auth("my_token") {
	///     println!("{err:?}");
	/// };
	/// ```
	pub fn auth(&self, token: &str) -> Result<(), PaperClientError> {
		for client in self.clients.iter() {
			client
				.lock()
				.expect("Could not obtain client.")
				.auth(token)?;
		}

		Ok(())
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
	/// let pool = PaperPool::new("paper://127.0.0.1:3145", 4).unwrap();
	///
	/// match pool.client().ping() {
	///     Ok(value) => println!("{value:?}"),
	///     Err(err) => println!("{err:?}"),
	/// };
	/// ```
	pub fn client(&self) -> MutexGuard<'_, PaperClient> {
		self.clients[self.get_index()]
			.lock()
			.expect("Could not obtain client.")
	}

	fn get_index(&self) -> usize {
		let index = self.index.load(Ordering::Relaxed);
		self.index
			.store((index + 1) % self.clients.len(), Ordering::Relaxed);
		index
	}
}
