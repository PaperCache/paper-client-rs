use std::net::TcpStream;

use paper_utils::{
	sheet::SheetBuilder,
	stream::{Buffer, StreamReader, StreamError},
	command::CommandByte,
	policy::PolicyByte,
};

use crate::{
	response::PaperClientResponse,
	policy::Policy,
	stats::Stats,
};

pub enum Command<'a> {
	Ping,
	Version,

	Get(&'a str),
	Set(&'a str, &'a Buffer, u32),
	Del(&'a str),

	Has(&'a str),
	Peek(&'a str),

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
					.write_buf(value)
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

	pub fn parse_buf_stream(&self, stream: &mut TcpStream) -> Result<PaperClientResponse, StreamError> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader.read_bool()?;
		let response = reader.read_buf()?;

		Ok(PaperClientResponse::new(is_ok, response))
	}

	pub fn parse_has_stream(&self, stream: &mut TcpStream) -> Result<PaperClientResponse<bool>, StreamError> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader.read_bool()?;
		let response = reader.read_bool()?;

		Ok(PaperClientResponse::new(is_ok, response))
	}

	pub fn parse_stats_stream(&self, stream: &mut TcpStream) -> Result<PaperClientResponse<Stats>, StreamError> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader.read_bool()?;

		let max_size = reader.read_u64()?;
		let used_size = reader.read_u64()?;

		let total_gets = reader.read_u64()?;
		let total_sets = reader.read_u64()?;
		let total_dels = reader.read_u64()?;

		let miss_ratio = reader.read_f64()?;

		let policy_byte = reader.read_u8()?;
		let uptime = reader.read_u64()?;

		let Ok(policy) = Policy::from_byte(policy_byte) else {
			return Err(StreamError::InvalidData);
		};

		let stats = Stats::new(
			max_size,
			used_size,

			total_gets,
			total_sets,
			total_dels,

			miss_ratio,

			policy,
			uptime
		);

		Ok(PaperClientResponse::<Stats>::new(is_ok, stats))
	}
}
