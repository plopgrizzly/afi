// Aldaron's Format Interface
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/audio.rs

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
