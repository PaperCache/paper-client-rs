use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum PaperClientError {
	#[error("Could not connect to paper server.")]
	InvalidAddress,

	#[error("Could not send command to server.")]
	InvalidCommand,

	#[error("Could not receive response from server.")]
	InvalidResponse,

	#[error("Connection was rejected by paper server.")]
	Rejected,

	#[error("An internal error occured.")]
	Internal,

	#[error("Disconnected from PaperServer.")]
	Disconnected,

	#[error("`{0}`")]
	CacheError(String),
}
