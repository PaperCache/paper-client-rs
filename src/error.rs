use std::error::Error;
use std::fmt::{Display, Formatter};
pub use paper_utils::error::PaperError;

#[derive(PartialEq, Debug)]
pub enum ErrorKind {
	InvalidAddress,
	InvalidStream,
	Disconnected,
	Internal,
}

#[derive(Debug)]
pub struct PaperClientError {
	kind: ErrorKind,
	message: String,
}

impl PaperClientError {
	pub fn new(kind: ErrorKind, message: &str) -> Self {
		PaperClientError {
			kind,
			message: message.to_owned(),
		}
	}

	pub fn kind(&self) -> &ErrorKind {
		&self.kind
	}
}

impl PaperError for PaperClientError {
	fn message(&self) -> &str {
		&self.message
	}
}

impl Error for PaperClientError {}

impl Display for PaperClientError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
