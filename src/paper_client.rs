use tokio::net::TcpStream;
pub use paper_core::error::PaperError;
use crate::error::{PaperClientError, ErrorKind};
use crate::response::PaperClientResponse;
use crate::command::Command;
use crate::policy::Policy;
use crate::stats::Stats;

pub struct PaperClient {
	stream: TcpStream,
}

impl PaperClient {
	/// Creates a new instance of the client and connects to the server.
	/// If a connection could not be established, an error is returned.
	///
	/// # Examples
	/// ```
	/// let client = PaperClient::new("127.0.0.1", 3145).unwrap();
	/// ```
	pub async fn new(host: &str, port: &u32) -> Result<Self, PaperClientError> {
		let addr = format!("{}:{}", host, port);

		let stream = match TcpStream::connect(addr).await {
			Ok(stream) => stream,

			Err(_) => {
				return Err(PaperClientError::new(
					ErrorKind::InvalidAddress,
					"Could not connect to paper server."
				));
			},
		};

		if let Err(_) = stream.set_nodelay(true) {
			return Err(PaperClientError::new(
				ErrorKind::Internal,
				"An internal error occured."
			));
		}

		let client = PaperClient {
			stream,
		};

		Ok(client)
	}

	/// Pings the server.
	///
	/// # Examples
	/// ```
	/// match client.ping() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn ping(&self) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Ping;

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Gets the cache version.
	///
	/// # Examples
	/// ```
	/// match client.version() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn version(&self) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Version;

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Gets the value of the supplied key from the cache.
	///
	/// # Examples
	/// ```
	/// match client.get("key") {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn get(&self, key: &str) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Get(key);

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Sets the supplied key, value, and ttl to the cache.
	///
	/// # Examples
	/// ```
	/// match client.set("key", "value", 0) {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn set(&self, key: &str, value: &str, ttl: &Option<u32>) -> Result<PaperClientResponse, PaperClientError> {
		let ttl_value = match ttl {
			Some(value) => value,
			None => &0,
		};

		let command = &Command::Set(key, value, &ttl_value);

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Deletes the value of the supplied key from the cache.
	///
	/// # Examples
	/// ```
	/// match client.del("key") {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn del(&self, key: &str) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Del(key);

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Wipes the contents of the cache.
	///
	/// # Examples
	/// ```
	/// match client.clear() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn wipe(&self) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Wipe;

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Resizes the cache to the supplied size.
	///
	/// # Examples
	/// ```
	/// match client.resize(10) {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn resize(&self, size: &u64) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Resize(size);

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Sets the cache's eviction policy.
	///
	/// # Examples
	/// ```
	/// match client.policy(&Policy::Lru) {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn policy(&self, policy: &Policy) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Policy(policy);

		self.send(&command).await?;
		self.receive(&command).await
	}

	/// Gets the cache statistics.
	///
	/// # Examples
	/// ```
	/// match client.stats() {
	///     Ok(response) => println!("{}: {:?}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub async fn stats(&self) -> Result<PaperClientResponse<Stats>, PaperClientError> {
		let command = &Command::Stats;

		self.send(&command).await?;
		self.receive_stats(&command).await
	}

	async fn send<'a>(&self, command: &Command<'a>) -> Result<(), PaperClientError> {
		if let Err(_) = command.to_stream(&self.stream).await {
			return Err(PaperClientError::new(
				ErrorKind::InvalidStream,
				"Could not send command to server."
			));
		}

		Ok(())
	}

	async fn receive<'a>(&self, command: &Command<'a>) -> Result<PaperClientResponse, PaperClientError> {
		if let Err(_) = self.stream.readable().await {
			return Err(PaperClientError::new(
				ErrorKind::Disconnected,
				"Disconnected from server."
			));
		}

		match command.parse_string_stream(&self.stream).await {
			Ok(response) => Ok(response),

			Err(_) => Err(PaperClientError::new(
				ErrorKind::InvalidStream,
				"Could not receive response from server."
			)),
		}
	}

	async fn receive_stats<'a>(&self, command: &Command<'a>) -> Result<PaperClientResponse<Stats>, PaperClientError> {
		if let Err(_) = self.stream.readable().await {
			return Err(PaperClientError::new(
				ErrorKind::Disconnected,
				"Disconnected from server."
			));
		}

		match command.parse_stats_stream(&self.stream).await {
			Ok(response) => Ok(response),

			Err(_) => Err(PaperClientError::new(
				ErrorKind::InvalidStream,
				"Could not receive response from server."
			)),
		}
	}
}
