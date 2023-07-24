use std::net::TcpStream;
pub use paper_utils::error::PaperError;

use crate::{
	error::{PaperClientError, ErrorKind},
	response::PaperClientResponse,
	command::Command,
	policy::Policy,
	stats::Stats,
};

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
	pub fn new(host: &str, port: u32) -> Result<Self, PaperClientError> {
		let addr = format!("{}:{}", host, port);

		let stream = match TcpStream::connect(addr) {
			Ok(stream) => stream,

			Err(_) => {
				return Err(PaperClientError::new(
					ErrorKind::InvalidAddress,
					"Could not connect to paper server."
				));
			},
		};

		if stream.set_nodelay(true).is_err() {
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
	pub fn ping(&mut self) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Ping;

		self.send(command)?;
		self.receive(command)
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
	pub fn version(&mut self) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Version;

		self.send(command)?;
		self.receive(command)
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
	pub fn get(&mut self, key: &str) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Get(key);

		self.send(command)?;
		self.receive(command)
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
	pub fn set(&mut self, key: &str, value: &str, ttl: Option<u32>) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Set(key, value, ttl.unwrap_or(0));

		self.send(command)?;
		self.receive(command)
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
	pub fn del(&mut self, key: &str) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Del(key);

		self.send(command)?;
		self.receive(command)
	}

	/// Wipes the contents of the cache.
	///
	/// # Examples
	/// ```
	/// match client.wipe() {
	///     Ok(response) => println!("{}: {}", response.is_ok(), repsonse.data()),
	///     Ok(err) => println!("{:?}", err),
	/// }
	/// ```
	pub fn wipe(&mut self) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Wipe;

		self.send(command)?;
		self.receive(command)
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
	pub fn resize(&mut self, size: u64) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Resize(size);

		self.send(command)?;
		self.receive(command)
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
	pub fn policy(&mut self, policy: Policy) -> Result<PaperClientResponse, PaperClientError> {
		let command = &Command::Policy(policy);

		self.send(command)?;
		self.receive(command)
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
	pub fn stats(&mut self) -> Result<PaperClientResponse<Stats>, PaperClientError> {
		let command = &Command::Stats;

		self.send(command)?;
		self.receive_stats(command)
	}

	fn send(&mut self, command: &Command<'_>) -> Result<(), PaperClientError> {
		command.to_stream(&mut self.stream).map_err(|_| {
			PaperClientError::new(
				ErrorKind::InvalidStream,
				"Could not send command to server."
			)
		})
	}

	fn receive(&mut self, command: &Command<'_>) -> Result<PaperClientResponse, PaperClientError> {
		command.parse_string_stream(&mut self.stream).map_err(|_| {
			PaperClientError::new(
				ErrorKind::InvalidStream,
				"Could not receive response from server."
			)
		})
	}

	fn receive_stats(&mut self, command: &Command<'_>) -> Result<PaperClientResponse<Stats>, PaperClientError> {
		command.parse_stats_stream(&mut self.stream).map_err(|_| {
			PaperClientError::new(
				ErrorKind::InvalidStream,
				"Could not receive response from server."
			)
		})
	}
}
