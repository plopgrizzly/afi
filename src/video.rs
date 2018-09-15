// "afi" - Aldaron's Format Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.  (See accompanying
// file LICENSE or copy at https://www.boost.org/LICENSE_1_0.txt)

use VFrame;
use std::collections::VecDeque;
use std::mem::transmute;

type Float = f32;

/// A linear HSVA value, can be created from sRGB value.
pub struct LHsva(pub f32, pub f32, pub f32, pub f32);

impl From<[u8; 4]> for LHsva {
	fn from(rgb: [u8; 4]) -> LHsva {
		// u8 S to f32 Linear Look Up Table
		const STOLINLUT: [f32; 256] = [
			0.0, 0.000303527, 0.000607054, 0.000910581,
			0.001214108, 0.001517635, 0.001821162, 0.0021246888,
			0.002428216, 0.0027317428, 0.00303527, 0.0033465358,
			0.0036765074, 0.004024717, 0.004391442, 0.0047769533,
			0.0051815165, 0.0056053917, 0.006048833, 0.0065120906,
			0.00699541, 0.007499032, 0.008023193, 0.008568126,
			0.009134059, 0.009721218, 0.010329823, 0.010960094,
			0.011612245, 0.012286488, 0.0129830325, 0.013702083,
			0.014443844, 0.015208514, 0.015996294, 0.016807375,
			0.017641954, 0.01850022, 0.019382361, 0.020288562,
			0.02121901, 0.022173885, 0.023153367, 0.024157632,
			0.02518686, 0.026241222, 0.027320892, 0.02842604,
			0.029556835, 0.030713445, 0.031896032, 0.033104766,
			0.034339808, 0.035601314, 0.03688945, 0.038204372,
			0.039546236, 0.0409152, 0.04231141, 0.04373503,
			0.045186203, 0.046665087, 0.048171826, 0.049706567,
			0.051269457, 0.052860647, 0.054480277, 0.05612849,
			0.05780543, 0.059511237, 0.061246052, 0.063010015,
			0.064803265, 0.06662594, 0.06847817, 0.070360094,
			0.07227185, 0.07421357, 0.07618538, 0.07818742,
			0.08021982, 0.08228271, 0.08437621, 0.08650046,
			0.08865558, 0.09084171, 0.093058966, 0.09530747,
			0.09758735, 0.099898726, 0.10224173, 0.104616486,
			0.107023105, 0.10946171, 0.11193243, 0.114435375,
			0.116970666, 0.11953843, 0.122138776, 0.12477182,
			0.12743768, 0.13013647, 0.13286832, 0.13563333,
			0.13843161, 0.14126329, 0.14412847, 0.14702727,
			0.14995979, 0.15292615, 0.15592647, 0.15896083,
			0.16202937, 0.1651322, 0.1682694, 0.17144111,
			0.1746474, 0.17788842, 0.18116425, 0.18447499,
			0.18782078, 0.19120169, 0.19461784, 0.19806932,
			0.20155625, 0.20507874, 0.20863687, 0.21223076,
			0.2158605, 0.2195262, 0.22322796, 0.22696587,
			0.23074006, 0.23455058, 0.23839757, 0.24228112,
			0.24620132, 0.25015828, 0.2541521, 0.25818285,
			0.26225066, 0.2663556, 0.2704978, 0.2746773,
			0.27889428, 0.28314874, 0.28744084, 0.29177064,
			0.29613826, 0.30054379, 0.3049873, 0.30946892,
			0.31398872, 0.31854677, 0.3231432, 0.3277781,
			0.33245152, 0.33716363, 0.34191442, 0.34670407,
			0.3515326, 0.35640013, 0.3613068, 0.3662526,
			0.3712377, 0.37626213, 0.38132602, 0.38642943,
			0.39157248, 0.39675522, 0.40197778, 0.4072402,
			0.4125426, 0.41788507, 0.42326766, 0.4286905,
			0.43415365, 0.43965718, 0.4452012, 0.4507858,
			0.45641103, 0.462077, 0.4677838, 0.47353148,
			0.47932017, 0.48514995, 0.49102086, 0.49693298,
			0.5028865, 0.50888133, 0.5149177, 0.52099556,
			0.5271151, 0.5332764, 0.5394795, 0.54572445,
			0.55201143, 0.5583404, 0.5647115, 0.57112485,
			0.57758045, 0.58407843, 0.59061885, 0.59720176,
			0.60382736, 0.61049557, 0.6172066, 0.6239604,
			0.63075715, 0.63759685, 0.6444797, 0.65140563,
			0.65837485, 0.6653873, 0.67244315, 0.6795425,
			0.6866853, 0.69387174, 0.7011019, 0.70837575,
			0.7156935, 0.7230551, 0.73046076, 0.7379104,
			0.7454042, 0.7529422, 0.7605245, 0.76815116,
			0.7758222, 0.7835378, 0.7912979, 0.7991027,
			0.80695224, 0.8148466, 0.82278574, 0.8307699,
			0.838799, 0.8468732, 0.8549926, 0.8631572,
			0.8713671, 0.8796224, 0.8879231, 0.8962694,
			0.9046612, 0.91309863, 0.92158186, 0.9301109,
			0.9386857, 0.9473065, 0.9559733, 0.9646863,
			0.9734453, 0.9822506, 0.9911021, 1.0,
		];
		const SIXTH: f32 = (1.0 / 6.0);

		// Linear RGBA
		let (r, g, b) = (
			STOLINLUT[rgb[0] as usize],
			STOLINLUT[rgb[1] as usize],
			STOLINLUT[rgb[2] as usize],
		);

		let a = rgb[3] as f32 * (1.0 / 255.0); // Alpha is always linear

		// Calculate hue.
		if r == g && g == b {
			LHsva(0.0, 0.0, r, a)
		} else if r > g {
			if r > b { // R is Max
				if b < g { // R is Max, B is Min
					let delta = r - b;
					let sat = if rgb[0] == 0 { 0.0 }
						else { delta / r };
					let hue = (g - b) / delta;
					LHsva(hue * SIXTH, sat, r, a)
				} else { // R is Max, G is Min
					let delta = r - g;
					let sat = if rgb[0] == 0 { 0.0 }
						else { delta / r };
					let hue = (g - b) / delta;
					LHsva(hue * SIXTH, sat, r, a)
				}
			} else { // B is Max, G is Min
				let delta = b - g;
				let sat = if rgb[1] == 0 { 0.0 }
					else { delta / b };
				let hue = 4.0 + (r - g) / delta;
				LHsva(hue * SIXTH, sat, b, a)
			}
		} else if g > b { // G is Max
			if r < b { // G is Max, R is Min
				let delta = g - r;
				let sat = if rgb[2] == 0 { 0.0 }
					else { delta / g };
				let hue = 2.0 + (b - r) / delta;
				LHsva(hue * SIXTH, sat, g, a)
			} else { // G is Max, B is Min
				let delta = g - b;
				let sat = if rgb[2] == 0 { 0.0 }
					else { delta / g };
				let hue = 2.0 + (b - r) / delta;
				LHsva(hue * SIXTH, sat, g, a)
			}
		} else { // B is Max, R is Min
			let delta = b - r;
			let sat = if rgb[1] == 0 { 0.0 }
				else { delta / b };
			let hue = 4.0 + (r - g) / delta;
			LHsva(hue * SIXTH, sat, b, a)
		}
	}
}

/// Convert floating point color channel to u8 color channel.
fn float_to_u8(a: Float) -> u8 {
	let t = a * 256.0; //0-256
	if t >= 255.0 { // Count non-continuous value as 255
		255
	} else {
		t.trunc() as u8 // 0-255
	}
}

/// Convert u8 color channel to floating point color channel.
fn u8_to_float(a: u8) -> Float {
	a as Float / 255.0 // range 0-255 => 0-1
}

/// S space to Linear space
fn s_to_linear(s: Float) -> Float {
	if s <= 0.04045 {
		s / 12.92
	} else {
		((s + 0.055) / 1.055).powf(2.4)
	}
}

/// S space to Linear space for u8s
fn s_to_linear_u8(s: u8) -> u8 {
	float_to_u8(s_to_linear(u8_to_float(s)))
}

/// Linear space to S space
fn linear_to_s(linear: Float) -> Float {
	if linear <= 0.0031308 {
		linear * 12.92
	} else {
		1.055 * linear.powf(1.0/2.4) - 0.055
	}
}

/// Linear space to S space for u8s
fn linear_to_s_u8(l: u8) -> u8 {
	float_to_u8(linear_to_s(u8_to_float(l)))
}

/// Convert sRGBA to lHSVA
fn srgba_to_lhsva(rgba: [u8; 4]) -> [u8; 4] {
	let r = s_to_linear(u8_to_float(rgba[0]));
	let g = s_to_linear(u8_to_float(rgba[1]));
	let b = s_to_linear(u8_to_float(rgba[2]));

	let max = r.max(g).max(b);
	let min = r.min(g).min(b);
	let delta = max - min;

	let hue = if delta != 0.0 {
		let mut hue = if r == max {
			(g - b) / delta
		} else if g == max {
			2.0 + (b - r) / delta
		} else {
			4.0 + (r - g) / delta
		};

		if hue < 0.0 {
			hue += 6.0;
		}
		hue / 6.0
	} else {
	        0.0
	};

	let sat = if max == 0.0 { 0.0 } else { (max - min) / max };
	let val = max;

	[float_to_u8(hue), float_to_u8(sat), float_to_u8(val), rgba[3]]
}

/// Convert lHSVA to sRGBA
fn lhsva_to_srgba(hsva: [u8; 4]) -> [u8; 4] {
	let fh = u8_to_float(hsva[0]) * 6.0; // Get range 0-6
	let h = fh as i8; // int 0-6
	let s = u8_to_float(hsva[1]);
	let v = u8_to_float(hsva[2]);

	if hsva[1] == 0 { // if saturation is 0, then it's gray
		return [hsva[2], hsva[2], hsva[2], hsva[3]];
	}

	let f = fh - (h as Float); // difference from rounding to 0-6
	let p = v * (1.0 - s);
	let q = v * (1.0 - s * f);
	let t = v * (1.0 - s * (1.0 - f));

	let (r, g, b) = match h {
		1 => (q, v, p),
		2 => (p, v, t),
		3 => (p, q, v),
		4 => (t, p, v),
		5 => (v, p, q),
		_ => (v, t, p), // 0 or 6
	};

	[
		float_to_u8(linear_to_s(r)),
		float_to_u8(linear_to_s(g)),
		float_to_u8(linear_to_s(b)),
		hsva[3]
	]
}

/// The format for the color channels of the `Video`.
#[derive(Copy, Clone, PartialEq)] #[repr(u8)]
pub enum ColorChannels {
	/// Grayscale color format, 1 channel (stored in red channel).
	Sgrayscale = 1u8,
	/// sRGB color format, 3 channels.
	Srgb = 3u8,
	/// sRGBA color format, 4 channels.
	Srgba = 4u8,

	/// sBGR color format, 3 channels.
	Sbgr = 3u8 + 5u8,
	/// sBGRA color format, 4 channels.
	Sbgra = 4u8 + 5u8,

	/// Linear Grayscale, 1 channel (red).
	Lgrayscale = 1u8 + 10u8,
	/// Linear RGB color format, 3 channels.
	Lrgb = 3u8 + 10u8,
	/// Linear RGBA color format, 4 channels.
	Lrgba = 4u8 + 10u8,

	/// Linear BGR color format, 3 channels.
	Lbgr = 3u8 + 15u8,
	/// Linear BGRA color format, 4 channels.
	Lbgra = 4u8 + 15u8,

	/// Linear HSB/HSV colorspace, 3 channels.
	Lhsv = 3u8 + 20u8,
	/// Linear HSBA/HSVA, 4 channels.
	Lhsva = 4u8 + 20u8,

	/// YCbCr color format, 3 channels
	YCbCr = 3u8 + 25u8,
}

impl Default for ColorChannels {
	fn default() -> ColorChannels { Srgba }
}

pub use ColorChannels::*;

impl ColorChannels {
	/// Convert a pixel to this format from another.
	pub fn from(self, from: ColorChannels, p: [u8; 4]) -> [u8; 4] {
		if self == from {
			p
		} else {
			self.srgba_to(from.to_srgba(p))
		}
	}

	/// Pack an sRGBA [u8; 4] into an sRGBA u32
	pub fn pack(p: [u8; 4]) -> u32 {
		let r = (p[0] as u32).rotate_right(0); 
		let g = (p[1] as u32).rotate_right(8);
		let b = (p[2] as u32).rotate_right(16);
		let a = (p[3] as u32).rotate_right(24);

		r | g | b | a
	}

	/// Unpack an sRGBA u32 into an sRGBA [u8; 4]
	pub fn unpack(p: u32) -> [u8; 4] {
		unsafe {
			transmute(p)
		}
	}

	/// Convert a pixel from sRGBA to this format.
	fn srgba_to(self, p: [u8; 4]) -> [u8; 4] {
		let [r, g, b, a] = p;
		match self {
			Sgrayscale => {
				let gray = (r as u16 + g as u16 + b as u16) / 3;
				[gray as u8, 255, 255, 255]
			},
			Srgb => [r, g, b, 255],
			Srgba => [r, g, b, a],
			Sbgr => [b, g, r, 255],
			Sbgra => [b, g, r, a],
			Lgrayscale => {
				let gray = (r as u16 + g as u16 + b as u16) / 3;
				[s_to_linear_u8(gray as u8), 255, 255, 255]
			}
			Lrgb => [
				s_to_linear_u8(r), s_to_linear_u8(g),
				s_to_linear_u8(b), 255
			],
			Lrgba => [
				s_to_linear_u8(r), s_to_linear_u8(g),
				s_to_linear_u8(b), a
			],
			Lbgr => [
				s_to_linear_u8(b), s_to_linear_u8(g),
				s_to_linear_u8(r), 255
			],
			Lbgra => [
				s_to_linear_u8(b), s_to_linear_u8(g),
				s_to_linear_u8(r), a
			],
			Lhsv => srgba_to_lhsva([r, g, b, 255]),
			Lhsva => srgba_to_lhsva([r, g, b, a]),
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

	/// Convert a pixel in this format to sRGBA.
	fn to_srgba(self, p: [u8; 4]) -> [u8; 4] {
		let [r, g, b, a] = p;
		match self {
			Sgrayscale => [r, r, r, 255],
			Srgb => [r, g, b, 255],
			Srgba => [r, g, b, a],
			Sbgr => [b, g, r, 255],
			Sbgra => [b, g, r, a],
			Lgrayscale => {
				let s = linear_to_s_u8(r);
				[s, s, s, 255]
			}
			Lrgb => [
				linear_to_s_u8(r), linear_to_s_u8(g),
				linear_to_s_u8(b), 255
			],
			Lrgba => [
				linear_to_s_u8(r), linear_to_s_u8(g),
				linear_to_s_u8(b), a
			],
			Lbgr => [
				linear_to_s_u8(b), linear_to_s_u8(g),
				linear_to_s_u8(r), 255
			],
			Lbgra => [
				linear_to_s_u8(b), linear_to_s_u8(g),
				linear_to_s_u8(r), a
			],
			Lhsv => lhsva_to_srgba([r, g, b, 255]),
			Lhsva => lhsva_to_srgba([r, g, b, a]),
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn linear_and_s_ranges() {
		let max_sl = s_to_linear(1.0);
		let max_ls = linear_to_s(1.0);
		let min_sl = s_to_linear(0.0);
		let min_ls = linear_to_s(0.0);
		assert!(min_sl >= 0.0);
		assert!(min_ls >= 0.0);
		assert!(max_sl <= 1.0);
		assert!(max_ls <= 1.0);
	}

	#[test]
	fn linear_and_s_equal() {
		let max_sl = s_to_linear(1.0);
		let max_ls = linear_to_s(max_sl);
		let min_sl = s_to_linear(0.0);
		let min_ls = linear_to_s(min_sl);
		assert_eq!(float_to_u8(1.0), float_to_u8(max_ls));
		assert_eq!(float_to_u8(0.0), float_to_u8(min_ls));
	}

	#[test]
	fn same_alpha() {
		let [_, _, _, a1] = Srgba.from(Lhsva, [0, 0, 0, 0]);
		let [_, _, _, a2] = Lhsva.from(Srgba, [0, 0, 0, 0]);
		assert_eq!(a1, 0);
		assert_eq!(a2, 0);
		let [_, _, _, a1] = Srgba.from(Lhsva, [0, 0, 0, 127]);
		let [_, _, _, a2] = Lhsva.from(Srgba, [0, 0, 0, 127]);
		assert_eq!(a1, 127);
		assert_eq!(a2, 127);
		let [_, _, _, a1] = Srgba.from(Lhsva, [0, 0, 0, 255]);
		let [_, _, _, a2] = Lhsva.from(Srgba, [0, 0, 0, 255]);
		assert_eq!(a1, 255);
		assert_eq!(a2, 255);
	}

	#[test]
	fn color_persist() {
		for a in 0..255 {
			let color = Srgba.from(Lhsva, Lhsva.from(Srgba, [255, 255, 255, a]));

			assert_eq!(color, [255, 255, 255, a]);
		}
	}
}
