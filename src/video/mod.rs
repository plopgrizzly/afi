// Copyright Jeron Lau 2017 - 2018.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

mod blend;

pub use self::blend::{blend, over};

use std::collections::VecDeque;
use std::mem::transmute;
use VFrame;

type Float = f32;

/// Convert floating point color channel to u8 color channel.
fn float_to_u8(a: Float) -> u8 {
    let t = a * 256.0; //0-256
    if t >= 255.0 {
        // Count non-continuous value as 255
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
        1.055 * linear.powf(1.0 / 2.4) - 0.055
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

    [
        float_to_u8(hue),
        float_to_u8(sat),
        float_to_u8(val),
        rgba[3],
    ]
}

/// Convert lHSVA to sRGBA
fn lhsva_to_srgba(hsva: [u8; 4]) -> [u8; 4] {
    let fh = u8_to_float(hsva[0]) * 6.0; // Get range 0-6
    let h = fh as i8; // int 0-6
    let s = u8_to_float(hsva[1]);
    let v = u8_to_float(hsva[2]);

    if hsva[1] == 0 {
        // if saturation is 0, then it's gray
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
        hsva[3],
    ]
}

/// The format for the color channels of the `Video`.
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
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
    fn default() -> ColorChannels {
        Srgba
    }
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
        unsafe { transmute(p) }
    }

    /// Convert a pixel from sRGBA to this format.
    fn srgba_to(self, p: [u8; 4]) -> [u8; 4] {
        let [r, g, b, a] = p;
        match self {
            Sgrayscale => {
                let gray = (r as u16 + g as u16 + b as u16) / 3;
                [gray as u8, 255, 255, 255]
            }
            Srgb => [r, g, b, 255],
            Srgba => [r, g, b, a],
            Sbgr => [b, g, r, 255],
            Sbgra => [b, g, r, a],
            Lgrayscale => {
                let gray = (r as u16 + g as u16 + b as u16) / 3;
                [s_to_linear_u8(gray as u8), 255, 255, 255]
            }
            Lrgb => [s_to_linear_u8(r), s_to_linear_u8(g), s_to_linear_u8(b), 255],
            Lrgba => [s_to_linear_u8(r), s_to_linear_u8(g), s_to_linear_u8(b), a],
            Lbgr => [s_to_linear_u8(b), s_to_linear_u8(g), s_to_linear_u8(r), 255],
            Lbgra => [s_to_linear_u8(b), s_to_linear_u8(g), s_to_linear_u8(r), a],
            Lhsv => srgba_to_lhsva([r, g, b, 255]),
            Lhsva => srgba_to_lhsva([r, g, b, a]),
            // From https://en.wikipedia.org/wiki/YCbCr#JPEG_conversion
            YCbCr => {
                let [r, g, b] = [r as f64, g as f64, b as f64];
                let y = (0.299 * r) + (0.587 * g) + (0.114 * b);
                let cb = 128.0 - (0.168736 * r) - (0.331264 * g) + (0.5 * b);
                let cr = 128.0 + (0.5 * r) - (0.418688 * g) - (0.081312 * b);
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
            Lrgb => [linear_to_s_u8(r), linear_to_s_u8(g), linear_to_s_u8(b), 255],
            Lrgba => [linear_to_s_u8(r), linear_to_s_u8(g), linear_to_s_u8(b), a],
            Lbgr => [linear_to_s_u8(b), linear_to_s_u8(g), linear_to_s_u8(r), 255],
            Lbgra => [linear_to_s_u8(b), linear_to_s_u8(g), linear_to_s_u8(r), a],
            Lhsv => lhsva_to_srgba([r, g, b, 255]),
            Lhsva => lhsva_to_srgba([r, g, b, a]),
            // From https://en.wikipedia.org/wiki/YCbCr#JPEG_conversion
            YCbCr => {
                let [y, cb, cr] = [r as f64, g as f64, b as f64];
                let r = y + 1.402 * (cr - 128.0);
                let g = y - 0.344136 * (cb - 128.0) - 0.714136 * (cr - 128.0);
                let b = y + 1.772 * (cb - 128.0);
                [r as u8, g as u8, b as u8, 255]
            }
        }
    }

    /// Return the number of channels.
    #[inline(always)]
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
    pub fn new(format: ColorChannels, wh: (u16, u16), n_frames: u32) -> Self {
        Video {
            wh,
            n_frames,
            format,
            frames: VecDeque::new(),
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
