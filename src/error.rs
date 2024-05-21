use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum PaperClientError {
	#[error("Invalid PaperCache address.")]
	InvalidAddress,

	#[error("Could not connect to PaperServer.")]
	UnreachableServer,

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
