// Aldaron's Format Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

//! Aldaron's Format Interface is a library developed by Plop Grizzly for
//! providing memory structures for graphics, audio, video and text.

#![no_std]
#![warn(missing_docs)]
#![doc(html_logo_url = "http://plopgrizzly.com/afi/icon.png",
	html_favicon_url = "http://plopgrizzly.com/afi/icon.ico",
	html_root_url = "http://plopgrizzly.com/afi/")]

extern crate ami;

use ami::Vec;

/// The errors that can be returned if `decode()` fails when loading graphics.
#[derive(Debug)]
pub enum GraphicDecodeErr {
	/// Not correct format. (bad header)
	IncorrectFormat,
	/// Dimensions are not numbers
	BadNum,
	/// Not yet implemented
	GrayscaleNYI,
	/// Not yet implemented
	IndexedNYI,
	/// Not yet implemented
	AGrayscaleNYI,
	/// Bits NYI
	BitsNYI,
}

impl ::core::fmt::Display for GraphicDecodeErr {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		write!(f, "Couldn't parse PNG because: {}", match *self {
			GraphicDecodeErr::IncorrectFormat => "Bad header",
			GraphicDecodeErr::BadNum => "Dimensions aren't numbers",
			GraphicDecodeErr::GrayscaleNYI => "NYI: Grayscale",
			GraphicDecodeErr::IndexedNYI => "NYI: Indexed",
			GraphicDecodeErr::AGrayscaleNYI => "NYI: AGrayscale",
			GraphicDecodeErr::BitsNYI => "NYI: bad bits",
		})
	}
}

/// Builder for `Graphic`
pub struct GraphicBuilder;

impl GraphicBuilder {
	/// Create a new `GraphicBuilder`
	pub fn new() -> GraphicBuilder {
		GraphicBuilder
	}

	/// Create a WH,[RGBA] graphic
	pub fn rgba(self, rgba: Vec<u32>) -> Graphic {
		Graphic { bgra: false, data: rgba }
	}

	/// Create an WH,[BGRA] graphic
	pub fn bgra(self, bgra: Vec<u32>) -> Graphic {
		Graphic { bgra: true, data: bgra }
	}
}

/// A graphic (image)
pub struct Graphic {
	bgra: bool,
	data: Vec<u32>,
}

impl Graphic {
	/// Convert `self` into a BGRA graphic.
	pub fn bgra(&mut self) {
		if !self.bgra {
			for i in &mut self.data.as_mut_slice()[2..] {
				*i = i.swap_bytes().rotate_right(8);
			}
		}

		self.bgra = true;
	}

	/// Convert `self` into a RGBA graphic.
	pub fn rgba(&mut self) {
		if self.bgra {
			for i in &mut self.data.as_mut_slice()[2..] {
				*i = i.swap_bytes().rotate_right(8);
			}
		}

		self.bgra = false;
	}

	/// Get the graphic as a slice `(w, h, [pixels])`
	pub fn as_slice(&self) -> &[u32] {
		self.data.as_slice()
	}
}

/// A sound or music.
pub struct Audio {
	hz: u16,
	samples: Vec<u16>,
}

impl Audio {
	/// Create a new `Audio` from samples.
	pub fn new(hz: u16, samples: Vec<u16>) -> Audio {
		Audio { hz, samples }
	}

	/// Get a sample at a specific index.
	pub fn sample(&self, index: usize) -> u16 {
		self.samples[index]
	}

	/// Get an index for a specific point in time since the beginning of the
	/// sound.
	pub fn index(&self, seconds: f32) -> usize {
		(seconds / (self.hz as f32)) as usize
	}
}

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
