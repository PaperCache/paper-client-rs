use std::net::TcpStream;
pub use paper_utils::stream::{Buffer, StreamError};

use crate::{
	error::PaperClientError,
	command::Command,
	policy::Policy,
	stats::Stats,
};

pub type PaperClientResult<T> = Result<T, PaperClientError>;

pub struct PaperClient {
	stream: TcpStream,
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
	pub fn new(host: &str, port: u32) -> PaperClientResult<Self> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn ping(&mut self) -> PaperClientResult<Buffer> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn version(&mut self) -> PaperClientResult<Buffer> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn get(&mut self, key: &str) -> PaperClientResult<Buffer> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn set(&mut self, key: &str, value: &Buffer, ttl: Option<u32>) -> PaperClientResult<Buffer> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn del(&mut self, key: &str) -> PaperClientResult<Buffer> {
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
	///     Ok(has) => println!("{}", has),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn has(&mut self, key: &str) -> PaperClientResult<bool> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn peek(&mut self, key: &str) -> PaperClientResult<Buffer> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn ttl(&mut self, key: &str, ttl: Option<u32>) -> PaperClientResult<Buffer> {
		let command = Command::Ttl(key, ttl.unwrap_or(0));

		self.send(&command)?;
		self.receive(&command)
	}

	/// Gets the size of the value of the supplied key from the cache in bytes.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.size("key") {
	///     Ok(size) => println!("{}", size),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn size(&mut self, key: &str) -> PaperClientResult<u64> {
		let command = Command::Size(key);

		self.send(&command)?;
		self.receive_size(&command)
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn wipe(&mut self) -> PaperClientResult<Buffer> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn resize(&mut self, size: u64) -> PaperClientResult<Buffer> {
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
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn policy(&mut self, policy: Policy) -> PaperClientResult<Buffer> {
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
	///     Ok(stats) => println!("{:?}", stats),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn stats(&mut self) -> PaperClientResult<Stats> {
		let command = Command::Stats;

		self.send(&command)?;
		self.receive_stats(&command)
	}

	fn send(&mut self, command: &Command<'_>) -> Result<(), PaperClientError> {
		command
			.to_stream(&mut self.stream)
			.map_err(|err| match err {
				StreamError::InvalidStream => PaperClientError::Disconnected,
				_ => PaperClientError::InvalidCommand,
			})
	}

	fn receive(&mut self, command: &Command<'_>) -> PaperClientResult<Buffer> {
		command.parse_buf_stream(&mut self.stream)
	}

	fn receive_has(&mut self, command: &Command<'_>) -> PaperClientResult<bool> {
		command.parse_has_stream(&mut self.stream)
	}

	fn receive_size(&mut self, command: &Command<'_>) -> PaperClientResult<u64> {
		command.parse_size_stream(&mut self.stream)
	}

	fn receive_stats(&mut self, command: &Command<'_>) -> PaperClientResult<Stats> {
		command.parse_stats_stream(&mut self.stream)
	}
}
