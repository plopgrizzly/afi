// Copyright Jeron Lau 2017 - 2018.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

//! # Aldaron's Format Interface
//! This crate provides APIs for audio and video (buffers, encoders/decoders)
//! Encoder/decoder crates can depend on this crate.  Here's a list of crates
//! that do:
//!
//! * [aci_png](https://crates.io/crates/aci_png) - Encode/Decode png & apng
//! * [aci_ppm](https://crates.io/crates/aci_ppm) - Encode/Decode ppm & pnm

#![warn(missing_docs)]
#![doc(
    html_logo_url = "http://plopgrizzly.com/afi/icon.png",
    html_favicon_url = "http://plopgrizzly.com/afi/icon.ico",
    html_root_url = "http://plopgrizzly.com/afi/"
)]

mod audio;
mod codec;
mod path_op;
mod video;

pub use audio::*;
pub use codec::*;
pub use path_op::*;
pub use video::*;
