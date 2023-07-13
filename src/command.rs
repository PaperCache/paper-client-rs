use std::net::TcpStream;
use paper_utils::sheet::SheetBuilder;
use paper_utils::stream::{StreamReader, StreamError, ErrorKind};
use crate::response::PaperClientResponse;
use crate::policy::Policy;
use crate::stats::Stats;

pub enum Command<'a> {
	Ping,
	Version,

	Get(&'a str),
	Set(&'a str, &'a str, u32),
	Del(&'a str),

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
					.write_u8(0)
					.to_sheet()
			},

			Command::Version => {
				SheetBuilder::new()
					.write_u8(1)
					.to_sheet()
			},

			Command::Get(key) => {
				SheetBuilder::new()
					.write_u8(2)
					.write_str(key)
					.to_sheet()
			},

			Command::Set(key, value, ttl) => {
				SheetBuilder::new()
					.write_u8(3)
					.write_str(key)
					.write_str(value)
					.write_u32(*ttl)
					.to_sheet()
			},

			Command::Del(key) => {
				SheetBuilder::new()
					.write_u8(4)
					.write_str(key)
					.to_sheet()
			},

			Command::Wipe => {
				SheetBuilder::new()
					.write_u8(5)
					.to_sheet()
			},

			Command::Resize(size) => {
				SheetBuilder::new()
					.write_u8(6)
					.write_u64(*size)
					.to_sheet()
			},

			Command::Policy(policy) => {
				let byte: u8 = match policy {
					Policy::Lfu => 0,
					Policy::Fifo => 1,
					Policy::Lru => 2,
					Policy::Mru => 3,
				};

				SheetBuilder::new()
					.write_u8(7)
					.write_u8(byte)
					.to_sheet()
			},

			Command::Stats => {
				SheetBuilder::new()
					.write_u8(8)
					.to_sheet()
			},
		};

		sheet.to_stream(stream)
	}

	pub fn parse_string_stream(&self, stream: &mut TcpStream) -> Result<PaperClientResponse<String>, StreamError> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader.read_bool()?;
		let response = reader.read_string()?;

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

		let policy_index = reader.read_u8()?;
		let uptime = reader.read_u64()?;

		let Ok(policy) = Policy::from_index(&policy_index) else {
			return Err(StreamError::new(
				ErrorKind::InvalidData,
				"Invalid policy index."
			));
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
