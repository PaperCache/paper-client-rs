/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use thiserror::Error;
use paper_utils::stream::StreamReader;

#[derive(Debug, PartialEq, Error)]
pub enum PaperClientError {
	#[error(transparent)]
	ServerError(#[from] PaperServerError),

	#[error(transparent)]
	CacheError(#[from] PaperCacheError),

	#[error("invalid PaperCache address")]
	InvalidAddress,

	#[error("could not connect to PaperServer")]
	UnreachableServer,

	#[error("could not send command to PaperServer")]
	InvalidCommand,

	#[error("could not parse supplied value as PaperValue")]
	InvalidValue,

	#[error("could not receive response from PaperServer")]
	InvalidResponse,

	#[error("an internal error occurred")]
	Internal,

	#[error("disconnected from PaperServer")]
	Disconnected,
}

#[derive(Debug, PartialEq, Error)]
pub enum PaperCacheError {
	#[error("an internal error occurred")]
	Internal,

	#[error("the key was not found in the cache")]
	KeyNotFound,

	#[error("the value size cannot be zero")]
	ZeroValueSize,

	#[error("the value size cannot exceed the cache size")]
	ExceedingValueSize,

	#[error("the cache size cannot be zero")]
	ZeroCacheSize,

	#[error("unconfigured policy")]
	UnconfiguredPolicy,

	#[error("invalid policy")]
	InvalidPolicy,
}

#[derive(Debug, PartialEq, Error)]
pub enum PaperServerError {
	#[error("an internal error occurred")]
	Internal,

	#[error("the maximum number of connections was exceeded")]
	MaxConnectionsExceeded,

	#[error("unauthorized")]
	Unauthorized,
}

impl PaperClientError {
	pub fn from_stream(mut reader: StreamReader) -> Self {
		let Ok(code) = reader.read_u8() else {
			return PaperClientError::InvalidResponse;
		};

		if code == 0 {
			let Ok(cache_code) = reader.read_u8() else {
				return PaperClientError::InvalidResponse;
			};

			let cache_error = PaperCacheError::from_code(cache_code);

			return PaperClientError::CacheError(cache_error);
		}

		let server_error = PaperServerError::from_code(code);

		PaperClientError::ServerError(server_error)
	}
}

impl PaperServerError {
	fn from_code(code: u8) -> Self {
		match code {
			2 => PaperServerError::MaxConnectionsExceeded,
			3 => PaperServerError::Unauthorized,

			_ => PaperServerError::Internal,
		}
	}
}

impl PaperCacheError {
	fn from_code(code: u8) -> Self {
		match code {
			1 => PaperCacheError::KeyNotFound,

			2 => PaperCacheError::ZeroValueSize,
			3 => PaperCacheError::ExceedingValueSize,

			4 => PaperCacheError::ZeroCacheSize,

			5 => PaperCacheError::UnconfiguredPolicy,
			6 => PaperCacheError::InvalidPolicy,

			_ => PaperCacheError::Internal,
		}
	}
}
