/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::sync::{
	Arc,
	atomic::{AtomicUsize, Ordering},
};

use tokio::sync::{Mutex, MutexGuard};

use crate::{addr::FromPaperAddr, async_client::AsyncPaperClient, error::PaperClientError};

#[derive(Debug, Clone)]
pub struct AsyncPaperPool {
	clients: Arc<Box<[Arc<Mutex<AsyncPaperClient>>]>>,
	index:   Arc<AtomicUsize>,
}

impl AsyncPaperPool {
	/// Creates a new instance of a pool of clients of size `size`.
	/// If a connection could not be established to any of the clients,
	/// a `PaperClientError` is returned.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperPool;
	///
	/// let pool = AsyncPaperPool::new("paper://127.0.0.1:3145", 4).await.unwrap();
	/// ```
	pub async fn new(
		paper_addr: impl FromPaperAddr,
		size: usize,
	) -> Result<Self, PaperClientError> {
		assert!(size > 0);

		let mut clients = Vec::new();

		for _ in 0..size {
			let client = AsyncPaperClient::new(paper_addr.clone()).await?;
			clients.push(Arc::new(Mutex::new(client)));
		}

		let pool = AsyncPaperPool {
			clients: Arc::new(clients.into_boxed_slice()),
			index:   Arc::new(AtomicUsize::default()),
		};

		Ok(pool)
	}

	/// Attempts to authorize each client with the supplied auth token.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperPool;
	///
	/// let pool = AsyncPaperPool::new("paper://127.0.0.1:3145", 4).await.unwrap();
	///
	/// if let Err(err) = pool.auth("my_token").await {
	///     println!("{err:?}");
	/// };
	/// ```
	pub async fn auth(&self, token: &str) -> Result<(), PaperClientError> {
		for client in self.clients.iter() {
			client.lock().await.auth(token).await?;
		}

		Ok(())
	}

	/// Obtains a guarded `PaperClient`. Use this client, then drop the
	/// reference (or allow it to go out of scope). Do not hold a reference
	/// to this client, otherwise the client will be unusable by other
	/// threads in the future.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperPool;
	///
	/// let pool = AsyncPaperPool::new("paper://127.0.0.1:3145", 4).await.unwrap();
	///
	/// match pool.client().ping().await {
	///     Ok(value) => println!("{value:?}"),
	///     Err(err) => println!("{err:?}"),
	/// };
	/// ```
	pub async fn client(&self) -> MutexGuard<'_, AsyncPaperClient> {
		self.clients[self.get_index()].lock().await
	}

	fn get_index(&self) -> usize {
		let index = self.index.load(Ordering::Relaxed);
		self.index
			.store((index + 1) % self.clients.len(), Ordering::Relaxed);
		index
	}
}
