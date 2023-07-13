use crate::error::{PaperClientError, ErrorKind};

#[derive(Debug, Clone, Copy)]
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

	pub fn from_index(index: u8) -> Result<Self, PaperClientError> {
		match index {
			0 => Ok(Policy::Lfu),
			1 => Ok(Policy::Fifo),
			2 => Ok(Policy::Lru),
			3 => Ok(Policy::Mru),

			_ => Err(PaperClientError::new(
				ErrorKind::Internal,
				"Invalid policy index."
			)),
		}
	}
}
