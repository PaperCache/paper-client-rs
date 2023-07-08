use crate::error::{PaperClientError, ErrorKind};

#[derive(Debug)]
pub enum Policy {
	Lfu,
	Lru,
	Mru,
}

impl Policy {
	pub fn id(&self) -> String {
		match self {
			Policy::Lfu => "lfu".to_owned(),
			Policy::Lru => "lru".to_owned(),
			Policy::Mru => "mru".to_owned(),
		}
	}

	pub fn from_index(index: &u8) -> Result<Self, PaperClientError> {
		match index {
			0 => Ok(Policy::Lfu),
			1 => Ok(Policy::Lru),
			2 => Ok(Policy::Mru),

			_ => Err(PaperClientError::new(
				ErrorKind::Internal,
				"Invalid policy index."
			)),
		}
	}
}
