use crate::error::{PaperClientError, ErrorKind};

#[derive(Debug)]
pub enum Policy {
	Lru,
	Mru,
	Lfu,
}

impl Policy {
	pub fn id(&self) -> String {
		match self {
			Policy::Lru => "lru".to_owned(),
			Policy::Mru => "mru".to_owned(),
			Policy::Lfu => "lfu".to_owned(),
		}
	}

	pub fn from_index(index: &u8) -> Result<Self, PaperClientError> {
		match index {
			0 => Ok(Policy::Lru),
			1 => Ok(Policy::Mru),
			2 => Ok(Policy::Lfu),

			_ => Err(PaperClientError::new(
				ErrorKind::Internal,
				"Invalid policy index."
			)),
		}
	}
}
