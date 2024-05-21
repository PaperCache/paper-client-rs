use crate::{
	paper_client::PaperClientResult,
	error::PaperClientError,
};

pub trait FromPaperAddr: Clone {
	fn to_addr(&self) -> PaperClientResult<String>;
}

impl FromPaperAddr for &str {
	fn to_addr(&self) -> PaperClientResult<String> {
		if !self.starts_with("paper://") {
			return Err(PaperClientError::InvalidAddress);
		}

		Ok(self.replace("paper://", ""))
	}
}

impl FromPaperAddr for String {
	fn to_addr(&self) -> PaperClientResult<String> {
		self.as_str().to_addr()
	}
}
