/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

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
