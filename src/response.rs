pub struct PaperClientResponse<T = String> {
	is_ok: bool,
	data: T,
}

impl<T> PaperClientResponse<T> {
	pub fn new(is_ok: bool, data: T) -> Self {
		PaperClientResponse {
			is_ok,
			data,
		}
	}

	pub fn is_ok(&self) -> bool {
		self.is_ok
	}

	pub fn data(&self) -> &T {
		&self.data
	}
}
