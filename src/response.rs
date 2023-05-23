pub struct PaperClientResponse {
	is_ok: bool,
	data: String,
}

impl PaperClientResponse {
	pub fn new(is_ok: bool, data: String) -> PaperClientResponse {
		PaperClientResponse {
			is_ok,
			data,
		}
	}

	pub fn is_ok(&self) -> &bool {
		&self.is_ok
	}

	pub fn data(&self) -> &str {
		&self.data
	}
}
