use std::net::TcpStream;

use paper_utils::{
	sheet::SheetBuilder,
	stream::{StreamReader, StreamError},
	command::CommandByte,
	policy::PolicyByte,
};

use crate::{
	paper_client::PaperClientResult,
	error::PaperClientError,
	value::PaperValue,
	policy::Policy,
	stats::Stats,
};

pub enum Command<'a> {
	Ping,
	Version,

	Auth(&'a str),

	Get(&'a str),
	Set(&'a str, PaperValue, u32),
	Del(&'a str),

	Has(&'a str),
	Peek(&'a str),
	Ttl(&'a str, u32),
	Size(&'a str),

	Wipe,

	Resize(u64),
	Policy(Policy),

	Stats,
}

impl<'a> Command<'a> {
	pub fn to_stream(&self, stream: &mut TcpStream) -> Result<(), StreamError> {
		let sheet = match self {
			Command::Ping => {
				SheetBuilder::new()
					.write_u8(CommandByte::PING)
					.to_sheet()
			},

			Command::Version => {
				SheetBuilder::new()
					.write_u8(CommandByte::VERSION)
					.to_sheet()
			},

			Command::Auth(token) => {
				SheetBuilder::new()
					.write_u8(CommandByte::AUTH)
					.write_str(token)
					.to_sheet()
			},

			Command::Get(key) => {
				SheetBuilder::new()
					.write_u8(CommandByte::GET)
					.write_str(key)
					.to_sheet()
			},

			Command::Set(key, value, ttl) => {
				SheetBuilder::new()
					.write_u8(CommandByte::SET)
					.write_str(key)
					.write_buf(value.into())
					.write_u32(*ttl)
					.to_sheet()
			},

			Command::Del(key) => {
				SheetBuilder::new()
					.write_u8(CommandByte::DEL)
					.write_str(key)
					.to_sheet()
			},

			Command::Has(key) => {
				SheetBuilder::new()
					.write_u8(CommandByte::HAS)
					.write_str(key)
					.to_sheet()
			},

			Command::Peek(key) => {
				SheetBuilder::new()
					.write_u8(CommandByte::PEEK)
					.write_str(key)
					.to_sheet()
			},

			Command::Ttl(key, ttl) => {
				SheetBuilder::new()
					.write_u8(CommandByte::TTL)
					.write_str(key)
					.write_u32(*ttl)
					.to_sheet()
			},

			Command::Size(key) => {
				SheetBuilder::new()
					.write_u8(CommandByte::SIZE)
					.write_str(key)
					.to_sheet()
			},

			Command::Wipe => {
				SheetBuilder::new()
					.write_u8(CommandByte::WIPE)
					.to_sheet()
			},

			Command::Resize(size) => {
				SheetBuilder::new()
					.write_u8(CommandByte::RESIZE)
					.write_u64(*size)
					.to_sheet()
			},

			Command::Policy(policy) => {
				let byte: u8 = match policy {
					Policy::Lfu => PolicyByte::LFU,
					Policy::Fifo => PolicyByte::FIFO,
					Policy::Lru => PolicyByte::LRU,
					Policy::Mru => PolicyByte::MRU,
				};

				SheetBuilder::new()
					.write_u8(CommandByte::POLICY)
					.write_u8(byte)
					.to_sheet()
			},

			Command::Stats => {
				SheetBuilder::new()
					.write_u8(CommandByte::STATS)
					.to_sheet()
			},
		};

		sheet.to_stream(stream)
	}

	pub fn parse_buf_stream(&self, stream: &mut TcpStream) -> PaperClientResult<PaperValue> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader.read_bool().map_err(|_| PaperClientError::InvalidResponse)?;
		let buf = reader.read_buf().map_err(|_| PaperClientError::InvalidResponse)?;

		match is_ok {
			true => Ok(buf.into()),

			false => {
				let message = String::from_utf8(buf.to_vec())
					.map_err(|_| PaperClientError::InvalidResponse)?;

				Err(PaperClientError::CacheError(message))
			},
		}
	}

	pub fn parse_has_stream(&self, stream: &mut TcpStream) -> PaperClientResult<bool> {
		let mut reader = StreamReader::new(stream);

		match reader.read_bool().map_err(|_| PaperClientError::InvalidResponse)? {
			true => Ok(reader.read_bool().map_err(|_| PaperClientError::InvalidResponse)?),

			false => {
				let buf = reader.read_buf().map_err(|_| PaperClientError::InvalidResponse)?;

				let message = String::from_utf8(buf.to_vec())
					.map_err(|_| PaperClientError::InvalidResponse)?;

				Err(PaperClientError::CacheError(message))
			},
		}
	}

	pub fn parse_size_stream(&self, stream: &mut TcpStream) -> PaperClientResult<u64> {
		let mut reader = StreamReader::new(stream);

		match reader.read_bool().map_err(|_| PaperClientError::InvalidResponse)? {
			true => Ok(reader.read_u64().map_err(|_| PaperClientError::InvalidResponse)?),

			false => {
				let buf = reader.read_buf().map_err(|_| PaperClientError::InvalidResponse)?;

				let message = String::from_utf8(buf.to_vec())
					.map_err(|_| PaperClientError::InvalidResponse)?;

				Err(PaperClientError::CacheError(message))
			},
		}
	}

	pub fn parse_stats_stream(&self, stream: &mut TcpStream) -> PaperClientResult<Stats> {
		let mut reader = StreamReader::new(stream);

		match reader.read_bool().map_err(|_| PaperClientError::InvalidResponse)? {
			true => {
				let max_size = reader.read_u64().map_err(|_| PaperClientError::InvalidResponse)?;
				let used_size = reader.read_u64().map_err(|_| PaperClientError::InvalidResponse)?;

				let total_gets = reader.read_u64().map_err(|_| PaperClientError::InvalidResponse)?;
				let total_sets = reader.read_u64().map_err(|_| PaperClientError::InvalidResponse)?;
				let total_dels = reader.read_u64().map_err(|_| PaperClientError::InvalidResponse)?;

				let miss_ratio = reader.read_f64().map_err(|_| PaperClientError::InvalidResponse)?;

				let policy_byte = reader.read_u8().map_err(|_| PaperClientError::InvalidResponse)?;
				let uptime = reader.read_u64().map_err(|_| PaperClientError::InvalidResponse)?;

				let policy = Policy::from_byte(policy_byte)
					.map_err(|_| PaperClientError::InvalidResponse)?;

				let stats = Stats::new(
					max_size,
					used_size,

					total_gets,
					total_sets,
					total_dels,

					miss_ratio,

					policy,
					uptime,
				);

				Ok(stats)
			},

			false => {
				let buf = reader.read_buf().map_err(|_| PaperClientError::InvalidResponse)?;

				let message = String::from_utf8(buf.to_vec())
					.map_err(|_| PaperClientError::InvalidResponse)?;

				Err(PaperClientError::CacheError(message))
			},
		}
	}
}
