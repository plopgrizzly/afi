// Aldaron's Format Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

//! Aldaron's Format Interface is a library developed by Plop Grizzly for
//! reading and writing different formats.

#![no_std]
#![warn(missing_docs)]
#![doc(html_logo_url = "http://plopgrizzly.com/afi/icon.png",
	html_favicon_url = "http://plopgrizzly.com/afi/icon.ico",
	html_root_url = "http://plopgrizzly.com/afi/")]

extern crate ami;

use ami::Vec;

/// A graphic.  Format: `(W, H, [RGBA Pixels])`
pub struct Graphic(pub Vec<u32>);

/// Sound.  Format: `(Hz, [Samples])`
pub struct Sound(pub Vec<u16>);

/// Text.  Format `([Characters])`
pub struct Text(pub Vec<u8>);
