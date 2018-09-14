// "afi" - Aldaron's Format Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.  (See accompanying
// file LICENSE or copy at https://www.boost.org/LICENSE_1_0.txt)

use Video;
use Audio;
use ColorChannels;

/// Index for a frame.
#[derive(Copy, Clone)] pub struct Index(pub u32);
/// A Video Frame
#[derive(Clone)] pub struct VFrame(pub Vec<u8>);
/// An audio frame (2000 samples = 1/24 of a second)
#[derive(Clone)] pub struct AFrame(pub [i16; 2000]);

impl Index {
	/// Convert Index to seconds
	pub fn to_seconds(self) -> f32 {
		(self.0 as f32) / 24.0
	}
}

impl VFrame {
	/// Get RGBA from color format and index.
	pub fn get_rgba(&self, format: ColorChannels, index: usize) -> [u8; 4] {
		let mut rgba = [255u8; 4];
		let channels = format.n_channels();

		for i in 0..channels {
			rgba[i] = self.0[index * channels + i];
		}

		rgba
	}

	/// Set RGBA for color format and index.
	pub fn set_rgba(&mut self, format: ColorChannels, index: usize,
		rgba: [u8; 4])
	{
		let channels = format.n_channels();

		for i in 0..channels {
			self.0[index * channels + i] = rgba[i];
		}
	}
}

/// A trait for implementing encoding video (use only with non-audio formats).
pub trait EncoderV where Self: Sized {
	/// Create a new encoder for this video.
	fn new(video: &Video) -> Self;
	/// Encode a frame (24fps) and return appended data.
	fn run(&mut self, frame: &VFrame) -> Vec<u8>;
	/// Finish the encoding and return appended data.
	fn end(self) -> Vec<u8>;
}

/// A trait for implementing encoding audio.
pub trait EncoderA where Self: Sized {
	/// Create a new encoder for this audio.
	fn new(audio: &Audio) -> Self;
	/// Encode a frame (2000 samples / 24fps) and return appended data.
	fn run(&mut self, audio: &mut Audio) -> Vec<u8>;
	/// Finish the encoding and return appended data.
	fn end(self) -> Vec<u8>;
}

/// A trait for implementing encoding audio and video together.
pub trait EncoderAV where Self: Sized {
	/// Create a new encoder for this audiovideo
	fn new(video: &Video, audio: &Audio) -> Self;
	/// Encode a frame (2000 samples / 24fps) and return appended data.
	fn run(&mut self, audio: &mut Audio, video: &mut Video) -> Vec<u8>;
	/// Finish the encoding and return appended data.
	fn end(self) -> Vec<u8>;
}

/// A trait for implementing decoding audio, video or both.
pub trait Decoder<T> where Self: Sized {
	/// Create a new decoder for this format.  `None` is returned when the
	/// decoder can't handle `data`'s format: try a different decoder.
	fn new(data: T, colors: ColorChannels) -> Option<Self>;
	/// Decode a frame.  `None` is returned if the file is corrupt,
	/// `Some(true)` if it succeeded, and `Some(false)` if it can't add
	/// anymore frames because the input file ended.  `audio` and `video`
	/// should initially be set to `None`.
	fn run(&mut self, audio: &mut Option<Audio>, video: &mut Option<Video>)
		-> Option<bool>;
	/// Get the frame number (24 frames per second).
	fn get(&self) -> Index;
	/// Set the frame number to seek forward or backwards.
	fn set(&mut self, index: Index);
}
