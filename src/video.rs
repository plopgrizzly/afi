// "afi" - Aldaron's Format Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.  (See accompanying
// file LICENSE or copy at https://www.boost.org/LICENSE_1_0.txt)

use VFrame;
use std::collections::VecDeque;
use std::mem::transmute;

/// The format for the color channels of the `Video`.
#[derive(Copy, Clone, PartialEq)] #[repr(u8)]
pub enum ColorChannels {
	/// Grayscale color format, 1 channel (stored in red channel).
	Grayscale = 1u8,
	/// RGB color format, 3 channels.
	Rgb = 3u8,
	/// RGBA color format, 4 channels.
	Rgba = 4u8,
	/// BGR color format, 3 channels.
	Bgr = 8u8,
	/// BGRA color format, 4 channels.
	Bgra = 9u8,
	/// YCbCr color format, 3 channels
	YCbCr = 13u8,
}

impl Default for ColorChannels {
	fn default() -> ColorChannels { Rgba }
}

pub use ColorChannels::*;

impl ColorChannels {
	/// Convert a pixel to this format from another.
	pub fn from(self, from: ColorChannels, p: [u8; 4]) -> [u8; 4] {
		if self == from {
			p
		} else {
			self.rgba_to(from.to_rgba(p))
		}
	}

	/// Pack an RGBA [u8; 4] into an RGBA u32
	pub fn pack(p: [u8; 4]) -> u32 {
		let r = (p[0] as u32).rotate_right(0); 
		let g = (p[1] as u32).rotate_right(8);
		let b = (p[2] as u32).rotate_right(16);
		let a = (p[3] as u32).rotate_right(24);

		r | g | b | a
	}

	/// Unpack an RGBA u32 into an RGBA [u8; 4]
	pub fn unpack(p: u32) -> [u8; 4] {
		unsafe {
			transmute(p)
		}
	}

	/// Convert a pixel from RGBA to this format.
	fn rgba_to(self, p: [u8; 4]) -> [u8; 4] {
		let [r, g, b, a] = p;
		match self {
			Grayscale => {
				let gray = (r as u16 + g as u16 + b as u16) / 3;
				[gray as u8, 255, 255, 255]
			},
			Rgb => [r, g, b, 255],
			Rgba => [r, g, b, a],
			Bgr => [b, g, r, 255],
			Bgra => [b, g, r, a],
			// From https://en.wikipedia.org/wiki/YCbCr#JPEG_conversion
			YCbCr => {
				let [r, g, b] = [r as f64, g as f64, b as f64];
				let y = (0.299 * r) + (0.587 * g) + (0.114 * b);
				let cb = 128.0 - (0.168736 * r) - (0.331264 * g)
					+ (0.5  * b);
				let cr = 128.0 + (0.5 * r) - (0.418688 * g)
					- (0.081312 * b);
				[y as u8, cb as u8, cr as u8, 255]
			}
		}
	}

	/// Convert a pixel in this format to RGBA.
	fn to_rgba(self, p: [u8; 4]) -> [u8; 4] {
		let [r, g, b, a] = p;
		match self {
			Grayscale => [r, r, r, r],
			Rgb => [r, g, b, 255],
			Rgba => [r, g, b, a],
			Bgr => [b, g, r, 255],
			Bgra => [b, g, r, a],
			// From https://en.wikipedia.org/wiki/YCbCr#JPEG_conversion
			YCbCr => {
				let [y, cb, cr] = [r as f64, g as f64, b as f64];
				let r = y + 1.402 * (cr - 128.0);
				let g = y - 0.344136 * (cb - 128.0) - 0.714136 *
					(cr - 128.0);
				let b = y + 1.772 * (cb - 128.0);
				[r as u8, g as u8, b as u8, 255]
			}
		}
	}

	/// Return the number of channels.
	pub fn n_channels(self) -> usize {
		(self as u8 % 5) as usize
	}
}

/// A Video Buffer (24fps).
pub struct Video {
	format: ColorChannels,
	wh: (u16, u16),
	n_frames: u32, // number of frames in the whole video.
	frames: VecDeque<VFrame>,
}

impl Video {
	/// Create a new video buffer.
	pub fn new(format: ColorChannels, wh: (u16, u16), n_frames: u32) -> Self
	{
		Video {
			wh, n_frames, format, frames: VecDeque::new(),
		}
	}

	/// Get the width and height of the video.
	pub fn wh(&self) -> (u16, u16) {
		self.wh
	}

	/// Get the video buffer's color format.
	pub fn format(&self) -> ColorChannels {
		self.format
	}

	/// Add frame to the buffer.
	pub fn add(&mut self, data: VFrame) {
		self.frames.push_back(data);
	}

	/// Return the number frames in the buffer.
	pub fn len(&self) -> u32 {
		self.frames.len() as u32
	}

	/// Returns pixels for the next frame on the Queue.
	pub fn pop(&mut self) -> Option<VFrame> {
		Some(self.frames.pop_front()?)
	}

	/// Return the number of channels.
	pub fn n_channels(&self) -> usize {
		self.format.n_channels()
	}

	/// Return the total number of frames in the video.
	pub fn frames(&self) -> u32 {
		self.n_frames
	}
}
