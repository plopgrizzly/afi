// Aldaron's Format Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/text.rs

use ami::Vec;

/// Some text.
pub struct Text {
	bytes: Vec<u8>
}

impl Text {
	/// Create a new `Text` from bytes.
	pub fn new(bytes: Vec<u8>) -> Text {
		Text { bytes }
	}

	/// Get bytes from the `Text`.
	pub fn bytes(&self) -> &[u8] {
		self.bytes.as_slice()
	}
}
