use std::net::TcpStream;
use thiserror::Error;
use paper_utils::stream::Buffer;

use crate::{
	response::PaperClientResponse,
	command::Command,
	policy::Policy,
	stats::Stats,
};

pub struct PaperClient {
	stream: TcpStream,
}

#[derive(Debug, Error)]
pub enum PaperClientError {
	#[error("Could not connect to paper server.")]
	InvalidAddress,

	#[error("Could not send command to server.")]
	InvalidCommand,

	#[error("Could not receive response from server.")]
	InvalidResponse,

	#[error("Connection was rejected by paper server.")]
	Rejected,

	#[error("An internal error occured.")]
	Internal,
}

impl PaperClient {
	/// Creates a new instance of the client and connects to the server.
	/// If a connection could not be established, a `PaperClientError`
	/// is returned.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let client = PaperClient::new("127.0.0.1", 3145).unwrap();
	/// ```
	pub fn new(host: &str, port: u32) -> Result<Self, PaperClientError> {
		let addr = format!("{}:{}", host, port);

		let Ok(stream) = TcpStream::connect(addr) else {
			return Err(PaperClientError::InvalidAddress);
		};

		if stream.set_nodelay(true).is_err() {
			return Err(PaperClientError::Internal);
		}

		let mut client = PaperClient {
			stream,
		};

		if client.ping().is_err() {
			return Err(PaperClientError::Rejected);
		}

		Ok(client)
	}

	/// Pings the server.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.ping() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn ping(&mut self) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Ping;

		self.send(&command)?;
		self.receive(&command)
	}

	/// Gets the cache version.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.version() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn version(&mut self) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Version;

		self.send(&command)?;
		self.receive(&command)
	}

	/// Gets the value of the supplied key from the cache.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.get("key") {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn get(&mut self, key: &str) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Get(key);

		self.send(&command)?;
		self.receive(&command)
	}

	/// Sets the supplied key, value, and ttl to the cache.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// let value = "value"
	///     .as_bytes()
	///     .to_vec()
	///     .into_boxed_slice();
	///
	/// match client.set("key", &value, None) {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn set(&mut self, key: &str, value: &Buffer, ttl: Option<u32>) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Set(key, value, ttl.unwrap_or(0));

		self.send(&command)?;
		self.receive(&command)
	}

	/// Deletes the value of the supplied key from the cache.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.del("key") {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn del(&mut self, key: &str) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Del(key);

		self.send(&command)?;
		self.receive(&command)
	}

	/// Checks if the cache contains an object with the supplied key
	/// without altering the eviction order of the objects.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.has("key") {
	///     Ok(response) => println!("{}: {}", response.is_ok(), response.data()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn has(&mut self, key: &str) -> Result<PaperClientResponse<bool>, PaperClientError> {
		let command = Command::Has(key);

		self.send(&command)?;
		self.receive_has(&command)
	}

	/// Gets (peeks) the value of the supplied key from the cache without altering
	/// the eviction order of the objects.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.peek("key") {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn peek(&mut self, key: &str) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Peek(key);

		self.send(&command)?;
		self.receive(&command)
	}

	/// Sets the TTL associated with the supplied key.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.ttl("key", Some(5)) {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn ttl(&mut self, key: &str, ttl: Option<u32>) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Ttl(key, ttl.unwrap_or(0));

		self.send(&command)?;
		self.receive(&command)
	}

	/// Wipes the contents of the cache.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.wipe() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn wipe(&mut self) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Wipe;

		self.send(&command)?;
		self.receive(&command)
	}

	/// Resizes the cache to the supplied size.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.resize(10) {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn resize(&mut self, size: u64) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Resize(size);

		self.send(&command)?;
		self.receive(&command)
	}

	/// Sets the cache's eviction policy.
	///
	/// # Examples
	/// ```
	/// use paper_client::{PaperClient, Policy};
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.policy(Policy::Lru) {
	///     Ok(response) => println!("{}: {}", response.is_ok(), String::from_utf8(response.data().to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn policy(&mut self, policy: Policy) -> Result<PaperClientResponse, PaperClientError> {
		let command = Command::Policy(policy);

		self.send(&command)?;
		self.receive(&command)
	}

	/// Gets the cache statistics.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.stats() {
	///     Ok(response) => println!("{}: {:?}", response.is_ok(), response.data()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn stats(&mut self) -> Result<PaperClientResponse<Stats>, PaperClientError> {
		let command = Command::Stats;

		self.send(&command)?;
		self.receive_stats(&command)
	}

	fn send(&mut self, command: &Command<'_>) -> Result<(), PaperClientError> {
		command
			.to_stream(&mut self.stream)
			.map_err(|_| PaperClientError::InvalidCommand)
	}

	fn receive(&mut self, command: &Command<'_>) -> Result<PaperClientResponse, PaperClientError> {
		command
			.parse_buf_stream(&mut self.stream)
			.map_err(|_| PaperClientError::InvalidResponse)
	}

	fn receive_has(&mut self, command: &Command<'_>) -> Result<PaperClientResponse<bool>, PaperClientError> {
		command
			.parse_has_stream(&mut self.stream)
			.map_err(|_| PaperClientError::InvalidResponse)
	}

	fn receive_stats(&mut self, command: &Command<'_>) -> Result<PaperClientResponse<Stats>, PaperClientError> {
		command
			.parse_stats_stream(&mut self.stream)
			.map_err(|_| PaperClientError::InvalidResponse)
	}
}
