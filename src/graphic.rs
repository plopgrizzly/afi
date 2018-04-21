// graphic.rs -- Aldaron's Format Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

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

impl ::std::fmt::Display for GraphicDecodeErr {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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

	/// Create an RGBA graphic.
	pub fn rgba(self, w: u32, h: u32, data: Vec<u32>) -> Graphic {
		let bgra = false;
		assert!(data.len() as u32 / w == h
			&& data.len() as u32 % w == 0);
		Graphic { bgra, w, h, data }
	}

	/// Create a BGRA graphic.
	pub fn bgra(self, w: u32, h: u32, data: Vec<u32>) -> Graphic {
		let bgra = true;
		assert!(data.len() as u32 / w == h
			&& data.len() as u32 % w == 0);
		Graphic { bgra, w, h, data }
	}
}

/// A graphic (image)
#[derive(Clone)]
pub struct Graphic {
	bgra: bool,
	w: u32,
	h: u32,
	data: Vec<u32>,
}

impl Graphic {
	/// Convert `self` into a BGRA graphic.
	pub fn bgra(&mut self) {
		if !self.bgra {
			for i in self.data.as_mut_slice() {
				*i = i.swap_bytes().rotate_right(8);
			}
		}

		self.bgra = true;
	}

	/// Convert `self` into a RGBA graphic.
	pub fn rgba(&mut self) {
		if self.bgra {
			for i in self.data.as_mut_slice() {
				*i = i.swap_bytes().rotate_right(8);
			}
		}

		self.bgra = false;
	}

	/// Get the graphic as a slice `(w, h, [pixels])`
	pub fn as_slice(&self) -> (u32, u32, &[u32]) {
		(self.w, self.h, self.data.as_slice())
	}

	/// Get the graphic as a slice `(w, h, [r, g, b, a])`
	pub fn as_bytes(&self) -> (u32, u32, &[u8]) {
		let slice = self.as_slice();
		// 
		let bytes = unsafe {
			::std::slice::from_raw_parts_mut(slice.2.as_ptr()
				as *mut u8, slice.2.len() * 4)
		};

		(slice.0, slice.1, bytes)
	}
}

impl AsRef<Graphic> for Graphic {
	fn as_ref(&self) -> &Graphic {
		self
	}
}
