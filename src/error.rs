use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum PaperClientError {
	#[error("Could not connect to PaperServer.")]
	InvalidAddress,

	#[error("Could not send command to PaperServer.")]
	InvalidCommand,

	#[error("Could not receive response from PaperServer.")]
	InvalidResponse,

	#[error("Connection was rejected by PaperServer.")]
	Rejected,

	#[error("An internal error occured.")]
	Internal,

	#[error("Disconnected from PaperServer.")]
	Disconnected,

	#[error("{0}")]
	CacheError(String),
}
