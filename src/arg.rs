pub trait AsPaperKey {
	fn as_paper_key(&self) -> &str;
}

pub trait AsPaperAuthToken {
	fn as_paper_auth_token(&self) -> &str;
}

impl AsPaperKey for &str {
	fn as_paper_key(&self) -> &str {
		self
	}
}

impl AsPaperKey for String {
	fn as_paper_key(&self) -> &str {
		self
	}
}

impl AsPaperKey for &String {
	fn as_paper_key(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for &str {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for String {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for &String {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}
