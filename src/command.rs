/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{net::TcpStream, str::FromStr};

use paper_utils::{
	command::CommandByte,
	sheet::SheetBuilder,
	stream::{StreamError, StreamReader},
};

use crate::{
	error::PaperClientError,
	paper_client::PaperClientResult,
	policy::PaperPolicy,
	status::Status,
	value::PaperValue,
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
	Policy(PaperPolicy),

	Status,
}

impl Command<'_> {
	pub fn to_stream(&self, stream: &mut TcpStream) -> Result<(), StreamError> {
		let sheet = match self {
			Command::Ping => SheetBuilder::new()
				.write_u8(CommandByte::PING)
				.into_sheet(),

			Command::Version => SheetBuilder::new()
				.write_u8(CommandByte::VERSION)
				.into_sheet(),

			Command::Auth(token) => SheetBuilder::new()
				.write_u8(CommandByte::AUTH)
				.write_str(token)
				.into_sheet(),

			Command::Get(key) => SheetBuilder::new()
				.write_u8(CommandByte::GET)
				.write_str(key)
				.into_sheet(),

			Command::Set(key, value, ttl) => SheetBuilder::new()
				.write_u8(CommandByte::SET)
				.write_str(key)
				.write_buf(value.into())
				.write_u32(*ttl)
				.into_sheet(),

			Command::Del(key) => SheetBuilder::new()
				.write_u8(CommandByte::DEL)
				.write_str(key)
				.into_sheet(),

			Command::Has(key) => SheetBuilder::new()
				.write_u8(CommandByte::HAS)
				.write_str(key)
				.into_sheet(),

			Command::Peek(key) => SheetBuilder::new()
				.write_u8(CommandByte::PEEK)
				.write_str(key)
				.into_sheet(),

			Command::Ttl(key, ttl) => SheetBuilder::new()
				.write_u8(CommandByte::TTL)
				.write_str(key)
				.write_u32(*ttl)
				.into_sheet(),

			Command::Size(key) => SheetBuilder::new()
				.write_u8(CommandByte::SIZE)
				.write_str(key)
				.into_sheet(),

			Command::Wipe => SheetBuilder::new()
				.write_u8(CommandByte::WIPE)
				.into_sheet(),

			Command::Resize(size) => SheetBuilder::new()
				.write_u8(CommandByte::RESIZE)
				.write_u64(*size)
				.into_sheet(),

			Command::Policy(policy) => SheetBuilder::new()
				.write_u8(CommandByte::POLICY)
				.write_str(policy.to_string())
				.into_sheet(),

			Command::Status => SheetBuilder::new()
				.write_u8(CommandByte::STATUS)
				.into_sheet(),
		};

		sheet.write_to_stream(stream)
	}

	pub fn parse_stream(&self, stream: &mut TcpStream) -> PaperClientResult<()> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader
			.read_bool()
			.map_err(|_| PaperClientError::InvalidResponse)?;

		match is_ok {
			true => Ok(()),
			false => Err(PaperClientError::from_stream(reader)),
		}
	}

	pub fn parse_buf_stream(&self, stream: &mut TcpStream) -> PaperClientResult<PaperValue> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader
			.read_bool()
			.map_err(|_| PaperClientError::InvalidResponse)?;

		match is_ok {
			true => {
				let buf = reader
					.read_buf()
					.map_err(|_| PaperClientError::InvalidResponse)?;

				Ok(buf.into())
			},

			false => Err(PaperClientError::from_stream(reader)),
		}
	}

	pub fn parse_has_stream(&self, stream: &mut TcpStream) -> PaperClientResult<bool> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader
			.read_bool()
			.map_err(|_| PaperClientError::InvalidResponse)?;

		match is_ok {
			true => {
				let has = reader
					.read_bool()
					.map_err(|_| PaperClientError::InvalidResponse)?;

				Ok(has)
			},

			false => Err(PaperClientError::from_stream(reader)),
		}
	}

	pub fn parse_size_stream(&self, stream: &mut TcpStream) -> PaperClientResult<u32> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader
			.read_bool()
			.map_err(|_| PaperClientError::InvalidResponse)?;

		match is_ok {
			true => {
				let size = reader
					.read_u32()
					.map_err(|_| PaperClientError::InvalidResponse)?;

				Ok(size)
			},

			false => Err(PaperClientError::from_stream(reader)),
		}
	}

	pub fn parse_status_stream(&self, stream: &mut TcpStream) -> PaperClientResult<Status> {
		let mut reader = StreamReader::new(stream);

		let is_ok = reader
			.read_bool()
			.map_err(|_| PaperClientError::InvalidResponse)?;

		if is_ok {
			let pid = reader
				.read_u32()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			let max_size = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;
			let used_size = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;
			let num_objects = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			let rss = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;
			let hwm = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			let total_gets = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;
			let total_sets = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;
			let total_dels = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			let miss_ratio = reader
				.read_f64()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			let mut policies = Vec::<PaperPolicy>::new();
			let num_policies = reader
				.read_u32()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			for _ in 0..num_policies {
				let policy_str = reader
					.read_string()
					.map_err(|_| PaperClientError::InvalidResponse)?;
				let policy = PaperPolicy::from_str(&policy_str)
					.map_err(|_| PaperClientError::InvalidResponse)?;

				policies.push(policy);
			}

			let policy_str = reader
				.read_string()
				.map_err(|_| PaperClientError::InvalidResponse)?;
			let policy = PaperPolicy::from_str(&policy_str)
				.map_err(|_| PaperClientError::InvalidResponse)?;
			let is_auto_policy = reader
				.read_bool()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			let uptime = reader
				.read_u64()
				.map_err(|_| PaperClientError::InvalidResponse)?;

			let status = Status::new(
				pid,
				max_size,
				used_size,
				num_objects,
				rss,
				hwm,
				total_gets,
				total_sets,
				total_dels,
				miss_ratio,
				policies,
				policy,
				is_auto_policy,
				uptime,
			);

			Ok(status)
		} else {
			Err(PaperClientError::from_stream(reader))
		}
	}
}
