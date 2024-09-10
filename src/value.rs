use std::{
	string::FromUtf8Error,
	str::{self, Utf8Error},
};

pub struct PaperValue(Box<[u8]>);

impl From<Box<[u8]>> for PaperValue {
	fn from(value: Box<[u8]>) -> Self {
		PaperValue(value)
	}
}

impl From<&[u8]> for PaperValue {
	fn from(value: &[u8]) -> Self {
		let buf = value
			.to_vec()
			.into_boxed_slice();

		PaperValue(buf)
	}
}

impl From<Vec<u8>> for PaperValue {
	fn from(value: Vec<u8>) -> Self {
		PaperValue(value.into_boxed_slice())
	}
}

impl From<&str> for PaperValue {
	fn from(value: &str) -> Self {
		let buf = value
			.as_bytes()
			.to_vec()
			.into_boxed_slice();

		PaperValue(buf)
	}
}

impl From<String> for PaperValue {
	fn from(value: String) -> Self {
		value.as_str().into()
	}
}

impl From<&String> for PaperValue {
	fn from(value: &String) -> Self {
		value.as_str().into()
	}
}

impl From<PaperValue> for Box<[u8]> {
	fn from(value: PaperValue) -> Self {
		value.0
	}
}

impl<'a> From<&'a PaperValue> for &'a [u8] {
	fn from(value: &'a PaperValue) -> Self {
		&value.0
	}
}

impl From<PaperValue> for Vec<u8> {
	fn from(value: PaperValue) -> Self {
		value.0.to_vec()
	}
}

impl<'a> TryFrom<&'a PaperValue> for &'a str {
	type Error = Utf8Error;

	fn try_from(value: &'a PaperValue) -> Result<Self, Self::Error> {
		str::from_utf8(&value.0)
	}
}

impl TryFrom<PaperValue> for String {
	type Error = FromUtf8Error;

	fn try_from(value: PaperValue) -> Result<Self, Self::Error> {
		String::from_utf8(value.0.to_vec())
	}
}
