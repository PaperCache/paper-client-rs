use crate::error::{PaperClientError, ErrorKind};

#[derive(Debug)]
pub enum Policy {
	Lru,
	Mru,
}

impl Policy {
	pub fn id(&self) -> String {
		match self {
			Policy::Lru => "lru".to_owned(),
			Policy::Mru => "mru".to_owned(),
		}
	}

	pub fn from_id(id: &str) -> Result<Self, PaperClientError> {
		match id {
			"lru" => Ok(Policy::Lru),
			"mru" => Ok(Policy::Mru),

			_ => Err(PaperClientError::new(
				ErrorKind::Internal,
				"Invalid policy ID."
			)),
		}
	}
}
