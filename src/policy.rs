use paper_utils::policy::PolicyByte;
use crate::paper_client::PaperClientError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
	Lfu,
	Fifo,
	Lru,
	Mru,
}

impl Policy {
	pub fn id(&self) -> String {
		match self {
			Policy::Lfu => "lfu".to_owned(),
			Policy::Fifo => "fifo".to_owned(),
			Policy::Lru => "lru".to_owned(),
			Policy::Mru => "mru".to_owned(),
		}
	}

	pub fn from_byte(byte: u8) -> Result<Self, PaperClientError> {
		match byte {
			PolicyByte::LFU => Ok(Policy::Lfu),
			PolicyByte::FIFO => Ok(Policy::Fifo),
			PolicyByte::LRU => Ok(Policy::Lru),
			PolicyByte::MRU => Ok(Policy::Mru),

			_ => Err(PaperClientError::Internal),
		}
	}
}
