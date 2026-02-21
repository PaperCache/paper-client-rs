use paper_utils::stream::{AsyncStreamReader, StreamError};
use tokio::{io::BufStream, net::TcpStream};

use crate::{
	addr::FromPaperAddr,
	arg::{AsPaperAuthToken, AsPaperKey},
	command::Command,
	error::{PaperClientError, PaperClientResult},
	policy::PaperPolicy,
	status::Status,
	value::PaperValue,
};

const RECONNECT_MAX_ATTEMPTS: u8 = 3;

#[derive(Debug)]
pub struct AsyncPaperClient {
	addr: String,

	auth_token:         Option<String>,
	reconnect_attempts: u8,

	stream: BufStream<TcpStream>,
}

impl AsyncPaperClient {
	/// Creates a new instance of the client and connects to the server.
	/// If a connection could not be established, a `PaperClientError`
	/// is returned.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	/// ```
	pub async fn new(paper_addr: impl FromPaperAddr) -> PaperClientResult<Self> {
		let addr = paper_addr.to_addr()?;
		let stream = init_stream(&addr).await?;

		let mut client = AsyncPaperClient {
			addr,

			auth_token: None,
			reconnect_attempts: 0,

			stream,
		};

		client.handshake().await?;

		Ok(client)
	}

	/// Pings the server.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.ping().await {
	///     Ok(value) => println!("{value:?}"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn ping(&mut self) -> PaperClientResult<PaperValue> {
		self.process_value(&Command::Ping).await
	}

	/// Gets the cache version.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.version().await {
	///     Ok(value) => println!("{value:?}"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn version(&mut self) -> PaperClientResult<PaperValue> {
		self.process_value(&Command::Version).await
	}

	/// Attempts to authorize the connection with the supplied auth token. This
	/// must match the auth token specified in the server's configuration to be
	/// successful.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.auth("my_token").await {
	///     Ok(_) => println!("done"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn auth(&mut self, token: impl AsPaperAuthToken) -> PaperClientResult<()> {
		let auth_token = token.as_paper_auth_token();

		let command = Command::Auth(auth_token);
		let result = self.process(&command).await;

		self.auth_token = Some(auth_token.to_owned());

		result
	}

	/// Gets the value of the supplied key from the cache.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.get("key").await {
	///     Ok(value) => println!("{value:?}"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn get(&mut self, key: impl AsPaperKey) -> PaperClientResult<PaperValue> {
		let command = Command::Get(key.as_paper_key());
		self.process_value(&command).await
	}

	/// Sets the supplied key, value, and ttl to the cache.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.set("key", "value", None).await {
	///     Ok(_) => println!("done"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn set(
		&mut self,
		key: impl AsPaperKey,
		value: impl TryInto<PaperValue>,
		ttl: Option<u32>,
	) -> PaperClientResult<()> {
		let value: PaperValue = value
			.try_into()
			.map_err(|_| PaperClientError::InvalidValue)?;

		let command = Command::Set(key.as_paper_key(), value, ttl.unwrap_or(0));

		self.process(&command).await
	}

	/// Deletes the value of the supplied key from the cache.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.del("key").await {
	///     Ok(_) => println!("done"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn del(&mut self, key: impl AsPaperKey) -> PaperClientResult<()> {
		let command = Command::Del(key.as_paper_key());
		self.process(&command).await
	}

	/// Checks if the cache contains an object with the supplied key
	/// without altering the eviction order of the objects.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.has("key").await {
	///     Ok(has) => println!("{has}"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn has(&mut self, key: impl AsPaperKey) -> PaperClientResult<bool> {
		let command = Command::Has(key.as_paper_key());
		self.process_has(&command).await
	}

	/// Gets (peeks) the value of the supplied key from the cache without
	/// altering the eviction order of the objects.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.peek("key").await {
	///     Ok(value) => println!("{value:?}"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn peek(&mut self, key: impl AsPaperKey) -> PaperClientResult<PaperValue> {
		let command = Command::Peek(key.as_paper_key());
		self.process_value(&command).await
	}

	/// Sets the TTL associated with the supplied key.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.ttl("key", Some(5)).await {
	///     Ok(_) => println!("done"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn ttl(&mut self, key: impl AsPaperKey, ttl: Option<u32>) -> PaperClientResult<()> {
		let command = Command::Ttl(key.as_paper_key(), ttl.unwrap_or(0));
		self.process(&command).await
	}

	/// Gets the size of the value of the supplied key from the cache in bytes.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.size("key").await {
	///     Ok(size) => println!("{size}"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn size(&mut self, key: impl AsPaperKey) -> PaperClientResult<u32> {
		let command = Command::Size(key.as_paper_key());
		self.process_size(&command).await
	}

	/// Wipes the contents of the cache.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.wipe().await {
	///     Ok(_) => println!("done"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn wipe(&mut self) -> PaperClientResult<()> {
		self.process(&Command::Wipe).await
	}

	/// Resizes the cache to the supplied size.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.resize(10).await {
	///     Ok(_) => println!("done"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn resize(&mut self, size: u64) -> PaperClientResult<()> {
		let command = Command::Resize(size);
		self.process(&command).await
	}

	/// Sets the cache's eviction policy.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::{AsyncPaperClient, PaperPolicy};
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.policy(PaperPolicy::Lru).await {
	///     Ok(_) => println!("done"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn policy(&mut self, policy: PaperPolicy) -> PaperClientResult<()> {
		let command = Command::Policy(policy);
		self.process(&command).await
	}

	/// Gets the cache's status.
	///
	/// # Examples
	/// ```ignore
	/// use paper_client::AsyncPaperClient;
	///
	/// let mut client = AsyncPaperClient::new("paper://127.0.0.1:3145").await.unwrap();
	///
	/// match client.status().await {
	///     Ok(status) => println!("{status:?}"),
	///     Err(err) => println!("{err:?}"),
	/// }
	/// ```
	pub async fn status(&mut self) -> PaperClientResult<Status> {
		self.process_status(&Command::Status).await
	}

	async fn process(&mut self, command: &Command<'_>) -> PaperClientResult<()> {
		if let Err(err) = self.send(command).await
			&& matches!(err, PaperClientError::InvalidResponse)
		{
			self.reconnect_attempts += 1;
			self.reconnect().await?;
			return Box::pin(self.process(command)).await;
		}

		match self.receive(command).await {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect().await?;
				Box::pin(self.process(command)).await
			},

			err => err,
		}
	}

	async fn process_value(&mut self, command: &Command<'_>) -> PaperClientResult<PaperValue> {
		if let Err(err) = self.send(command).await
			&& matches!(err, PaperClientError::InvalidResponse)
		{
			self.reconnect_attempts += 1;
			self.reconnect().await?;
			return Box::pin(self.process_value(command)).await;
		}

		match self.receive_value(command).await {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect().await?;
				Box::pin(self.process_value(command)).await
			},

			err => err,
		}
	}

	async fn process_has(&mut self, command: &Command<'_>) -> PaperClientResult<bool> {
		if let Err(err) = self.send(command).await
			&& matches!(err, PaperClientError::InvalidResponse)
		{
			self.reconnect_attempts += 1;
			self.reconnect().await?;
			return Box::pin(self.process_has(command)).await;
		}

		match self.receive_has(command).await {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect().await?;
				Box::pin(self.process_has(command)).await
			},

			err => err,
		}
	}

	async fn process_size(&mut self, command: &Command<'_>) -> PaperClientResult<u32> {
		if let Err(err) = self.send(command).await
			&& matches!(err, PaperClientError::InvalidResponse)
		{
			self.reconnect_attempts += 1;
			self.reconnect().await?;
			return Box::pin(self.process_size(command)).await;
		}

		match self.receive_size(command).await {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect().await?;
				Box::pin(self.process_size(command)).await
			},

			err => err,
		}
	}

	async fn process_status(&mut self, command: &Command<'_>) -> PaperClientResult<Status> {
		if let Err(err) = self.send(command).await
			&& matches!(err, PaperClientError::InvalidResponse)
		{
			self.reconnect_attempts += 1;
			self.reconnect().await?;
			return Box::pin(self.process_status(command)).await;
		}

		match self.receive_status(command).await {
			Ok(response) => {
				self.reconnect_attempts = 0;
				Ok(response)
			},

			Err(PaperClientError::InvalidResponse) => {
				self.reconnect_attempts += 1;
				self.reconnect().await?;
				Box::pin(self.process_status(command)).await
			},

			err => err,
		}
	}

	async fn send(&mut self, command: &Command<'_>) -> PaperClientResult<()> {
		command
			.write_async(&mut self.stream)
			.await
			.map_err(|err| match err {
				StreamError::InvalidStream => PaperClientError::Disconnected,
				_ => PaperClientError::InvalidCommand,
			})
	}

	async fn receive(&mut self, command: &Command<'_>) -> PaperClientResult<()> {
		command.parse_reader_async(&mut self.stream).await
	}

	async fn receive_value(&mut self, command: &Command<'_>) -> PaperClientResult<PaperValue> {
		command
			.parse_buf_reader_async(&mut self.stream)
			.await
	}

	async fn receive_has(&mut self, command: &Command<'_>) -> PaperClientResult<bool> {
		command
			.parse_has_reader_async(&mut self.stream)
			.await
	}

	async fn receive_size(&mut self, command: &Command<'_>) -> PaperClientResult<u32> {
		command
			.parse_size_reader_async(&mut self.stream)
			.await
	}

	async fn receive_status(&mut self, command: &Command<'_>) -> PaperClientResult<Status> {
		command
			.parse_status_reader_async(&mut self.stream)
			.await
	}

	async fn handshake(&mut self) -> PaperClientResult<()> {
		let mut reader = AsyncStreamReader::new(&mut self.stream);

		let is_ok = reader
			.read_bool()
			.await
			.map_err(|_| PaperClientError::UnreachableServer)?;

		match is_ok {
			true => Ok(()),
			false => Err(PaperClientError::from_reader_async(reader).await),
		}
	}

	async fn reconnect(&mut self) -> PaperClientResult<()> {
		if self.reconnect_attempts > RECONNECT_MAX_ATTEMPTS {
			return Err(PaperClientError::Disconnected);
		}

		self.stream = init_stream(&self.addr).await?;
		self.handshake().await?;

		if let Some(token) = self.auth_token.clone() {
			Box::pin(self.auth(token)).await?;
		}

		Ok(())
	}
}

async fn init_stream(addr: &str) -> PaperClientResult<BufStream<TcpStream>> {
	let stream = TcpStream::connect(addr)
		.await
		.map_err(|_| PaperClientError::UnreachableServer)?;

	if stream.set_nodelay(true).is_err() {
		return Err(PaperClientError::Internal);
	}

	Ok(BufStream::new(stream))
}
