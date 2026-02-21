use bytes::{BufMut, Bytes, BytesMut};
use paper_utils::{command::CommandByte, stream::FALSE_INDICATOR};
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt, BufStream},
	net::TcpStream,
};

use crate::{
	addr::FromPaperAddr,
	arg::{AsPaperAuthToken, AsPaperKey},
	command::Command,
	error::{PaperClientError, PaperClientResult},
	policy::PaperPolicy,
	status::Status,
	value::PaperValue,
};

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
	/// ```
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
	/// ```
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
		let mut req = BytesMut::with_capacity(1);
		req.put_u8(CommandByte::PING);

		self.process_with_value(req.freeze()).await
	}

	async fn process_with_value(&mut self, req: Bytes) -> PaperClientResult<PaperValue> {
		self.send(req).await?;
		let res = self.receive_value().await?;

		self.reconnect_attempts = 0;
		Ok(res)
	}

	async fn send(&mut self, req: Bytes) -> PaperClientResult<()> {
		self.stream
			.write_all(&req)
			.await
			.map_err(|_| PaperClientError::Disconnected)?;

		self.stream.flush().await;

		Ok(())
	}

	async fn receive_value(&mut self) -> PaperClientResult<PaperValue> {
		todo!();
	}

	async fn handshake(&mut self) -> PaperClientResult<()> {
		let is_ok = self
			.stream
			.read_u8()
			.await
			.map(|value| value != FALSE_INDICATOR)
			.map_err(|_| PaperClientError::UnreachableServer)?;

		match is_ok {
			true => Ok(()),
			false => Err(PaperClientError::from_async_stream(&mut self.stream).await),
		}
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
