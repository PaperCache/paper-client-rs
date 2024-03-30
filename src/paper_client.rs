use std::net::TcpStream;
pub use paper_utils::stream::{Buffer, StreamError};

use crate::{
	error::PaperClientError,
	command::Command,
	policy::Policy,
	stats::Stats,
};

const RECONNECT_MAX_ATTEMPTS: u8 = 3;

pub type PaperClientResult<T> = Result<T, PaperClientError>;

pub struct PaperClient {
	addr: String,

	auth_token: Option<String>,
	reconnect_attempts: u8,

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
		let stream = init_stream(&addr)?;

		let mut client = PaperClient {
			addr,

			auth_token: None,
			reconnect_attempts: 0,

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
		self.process(&Command::Ping)
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
		self.process(&Command::Version)
	}

	/// Attempts to authorize the connection with the supplied auth token. This
	/// must match the auth token specified in the server's configuration to be
	/// successful.
	///
	/// # Examples
	/// ```
	/// use paper_client::PaperClient;
	///
	/// let mut client = PaperClient::new("127.0.0.1", 3145).unwrap();
	///
	/// match client.auth("my_token") {
	///     Ok(buf) => println!("{}", String::from_utf8(buf.to_vec()).unwrap()),
	///     Err(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn auth(&mut self, token: &str) -> PaperClientResult<Buffer> {
		let command = Command::Auth(token);
		let result = self.process(&command);

		self.auth_token = Some(token.to_owned());

		result
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
		self.process(&command)
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
		self.process(&command)
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
		self.process(&command)
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
		self.process_has(&command)
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
		self.process(&command)
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
		self.process(&command)
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
		self.process_size(&command)
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
		self.process(&Command::Wipe)
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
		self.process(&command)
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
		self.process(&command)
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
		self.process_stats(&Command::Stats)
	}

	fn process(&mut self, command: &Command<'_>) -> PaperClientResult<Buffer> {
		match self.send(command).and_then(|_| self.receive(command)) {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect()?;
				self.process(command)
			},

			err => err,
		}
	}

	fn process_has(&mut self, command: &Command<'_>) -> PaperClientResult<bool> {
		match self.send(command).and_then(|_| self.receive_has(command)) {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect()?;
				self.process_has(command)
			},

			err => err,
		}
	}

	fn process_size(&mut self, command: &Command<'_>) -> PaperClientResult<u64> {
		match self.send(command).and_then(|_| self.receive_size(command)) {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect()?;
				self.process_size(command)
			},

			err => err,
		}
	}

	fn process_stats(&mut self, command: &Command<'_>) -> PaperClientResult<Stats> {
		match self.send(command).and_then(|_| self.receive_stats(command)) {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect()?;
				self.process_stats(command)
			},

			err => err,
		}
	}

	fn send(&mut self, command: &Command<'_>) -> PaperClientResult<()> {
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

	fn reconnect(&mut self) -> PaperClientResult<()> {
		if self.reconnect_attempts > RECONNECT_MAX_ATTEMPTS {
			return Err(PaperClientError::Disconnected);
		}

		self.stream = init_stream(&self.addr)?;

		if let Some(token) = self.auth_token.clone() {
			self.auth(&token)?;
		}

		Ok(())
	}
}

fn init_stream(addr: &str) -> PaperClientResult<TcpStream> {
	let Ok(stream) = TcpStream::connect(addr) else {
		return Err(PaperClientError::InvalidAddress);
	};

	if stream.set_nodelay(true).is_err() {
		return Err(PaperClientError::Internal);
	}

	Ok(stream)
}
