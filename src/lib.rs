// Aldaron's Format Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Format Interface is a library developed by Plop Grizzly for
//! providing memory structures for graphics, audio, video and text.

#![warn(missing_docs)]
#![doc(html_logo_url = "http://plopgrizzly.com/afi/icon.png",
	html_favicon_url = "http://plopgrizzly.com/afi/icon.ico",
	html_root_url = "http://plopgrizzly.com/afi/")]

mod graphic;
mod audio;
mod text;

pub use self::graphic::{ GraphicDecodeErr, GraphicBuilder, Graphic };
pub use self::audio::{ Audio };
pub use self::text::{ Text };
