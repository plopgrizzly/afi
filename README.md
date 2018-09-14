# [Aldaron's Format Interface](https://crates.io/crates/afi)
This crate provides APIs for audio and video (buffers, encoders/decoders)
Encoder/decoder crates can depend on this crate.  Here's a list of codec crates:

* [aci_png](https://crates.io/crates/aci_png) - Encode/Decode png & apng
* [aci_ppm](https://crates.io/crates/aci_ppm) - Encode/Decode ppm & pnm

## Features
**afi**'s features:
* Video & Audio Buffer Structs.
* Convert between different color formats including YCbCr.
* Traits for realtime encoders and decoders.

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 1.0 (Future Features)
* Make sure that functionality is complete.
* Most encoder / decoder crates should be almost ready for 1.0.0 release.
* Publish 1.0.0

## Change Log
### 0.8
* Renamed `VFrame::sample_rgba` to `VFrame::get_rgba`.
* Added `VFrame::set_rgba`
* Added `PathOp` for vector graphics.
* Renamed `ColorChannels::Rgb` to `ColorChannels::Srgb`
* Renamed `ColorChannels::Rgba` to `ColorChannels::Srgba`
* Renamed `ColorChannels::Bgr` to `ColorChannels::Sbgra`
* Renamed `ColorChannels::Bgra` to `ColorChannels::Sbgra`
* Renamed `ColorChannels::Grayscale` to `ColorChannels::Sgrayscale`
* Added ``

### 0.7
* Replaced `Graphic` with `Video`.
* Added `Encoder*` and `Decoder` Traits.
* Added `AFrame` and `VFrame`.
* Added `AudioChannels` and `ColorChannels` along with conversion functions.
* Version now matches with codec crates.

## Developed by [Plop Grizzly](http://plopgrizzly.com)
