use paper_utils::stream::Buffer;

use crate::{
	paper_client::PaperClientResult,
	error::PaperClientError,
};

pub trait AsPaperKey {
	fn as_paper_key(&self) -> &str;
}

pub trait AsPaperAuthToken {
	fn as_paper_auth_token(&self) -> &str;
}

pub trait IntoPaperValue {
	fn into_paper_value(self) -> Buffer;
}

pub trait FromPaperValue {
	/// Parses the value returned by the cache into a UTF-8 string.
	///
	/// # Errors
	///
	/// This function returns an error if the value is not a UTF-8 string.
	fn into_string(self) -> PaperClientResult<String>;
}

impl AsPaperKey for &str {
	fn as_paper_key(&self) -> &str {
		self
	}
}

impl AsPaperKey for String {
	fn as_paper_key(&self) -> &str {
		self
	}
}

impl AsPaperKey for &String {
	fn as_paper_key(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for &str {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for String {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for &String {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}

impl IntoPaperValue for Buffer {
	fn into_paper_value(self) -> Buffer {
		self
	}
}

impl IntoPaperValue for Vec<u8> {
	fn into_paper_value(self) -> Buffer {
		self.into_boxed_slice()
	}
}

impl IntoPaperValue for &[u8] {
	fn into_paper_value(self) -> Buffer {
		self.to_vec().into_paper_value()
	}
}

impl IntoPaperValue for &str {
	fn into_paper_value(self) -> Buffer {
		self.as_bytes().into()
	}
}

impl IntoPaperValue for String {
	fn into_paper_value(self) -> Buffer {
		self.as_str().into_paper_value()
	}
}

impl IntoPaperValue for &String {
	fn into_paper_value(self) -> Buffer {
		self.as_str().into_paper_value()
	}
}

impl FromPaperValue for Buffer {
	fn into_string(self) -> PaperClientResult<String> {
		String::from_utf8(self.into_vec())
			.map_err(|_| PaperClientError::InvalidStringValue)
	}
}
