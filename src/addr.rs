/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::error::{PaperClientError, PaperClientResult};

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

impl FromPaperAddr for &String {
	fn to_addr(&self) -> PaperClientResult<String> {
		self.as_str().to_addr()
	}
}
