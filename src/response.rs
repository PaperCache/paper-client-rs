use std::fmt::Display;
use paper_utils::stream::Buffer;

pub struct PaperClientResponse<T = Buffer> {
	data: Result<T, String>,
}

trait BasicResponseData {}

impl<T> PaperClientResponse<T> {
	pub fn new(data: Result<T, String>) -> Self {
		PaperClientResponse {
			data,
		}
	}

	pub fn is_ok(&self) -> bool {
		self.data.is_ok()
	}

	pub fn data(&self) -> Result<&T, &str> {
		self.data.as_ref().map_err(|value| value.as_str())
	}
}

impl BasicResponseData for f64 {}
impl BasicResponseData for f32 {}
impl BasicResponseData for i64 {}
impl BasicResponseData for i32 {}
impl BasicResponseData for i16 {}
impl BasicResponseData for i8 {}
impl BasicResponseData for isize {}
impl BasicResponseData for u64 {}
impl BasicResponseData for u32 {}
impl BasicResponseData for u16 {}
impl BasicResponseData for u8 {}
impl BasicResponseData for usize {}
impl BasicResponseData for bool {}

impl<T> From<PaperClientResponse<T>> for PaperClientResponse
where
	T: BasicResponseData + Display,
{
	fn from(value: PaperClientResponse<T>) -> Self {
		let data = match value.data() {
			Ok(value) => Ok(format!("{value}")
				.into_bytes()
				.into_boxed_slice()),

			Err(err) => Err(err.to_owned()),
		};

		PaperClientResponse::new(data)
	}
}
