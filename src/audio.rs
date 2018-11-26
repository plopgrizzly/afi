// Copyright Jeron Lau 2017 - 2018.
// Dual-licensed under either the MIT License or the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at https://www.boost.org/LICENSE_1_0.txt)

use std::collections::VecDeque;
use AFrame;

/// Mono, Stereo or Surround.
pub enum AudioChannels {
    /// Mono = 1 channel (front center)
    Mono = 1,
    /// Stereo = 2 channels (front left, front right)
    Stereo = 2,
    /// Surround = 5 channels (front left, front right, front center,
    /// back left, back right)
    Surround = 5,
}

pub use AudioChannels::*;

impl Default for AudioChannels {
    fn default() -> AudioChannels {
        Stereo
    }
}

/// An Audio Buffer (48kHz/48,000hz).
pub struct Audio {
    /// Title
    pub title: String,
    /// Artist / Author
    pub artist: String,
    /// Album Artist
    pub album_artist: String,
    /// Album
    pub album: String,
    /// CD
    pub cd: String,
    /// Release Date / Release Year
    pub release: String,
    /// Track #
    pub track_number: String,
    /// Track Count
    pub track_count: String,
    /// Genre
    pub genre: String,
    /// Comment
    pub comment: String,
    /// Composer
    pub composer: String,
    /// Original artist
    pub orig_artist: String,
    /// Copyright / License
    pub copyright: String,
    /// Artist Website / URL
    pub url: String,
    /// Encoded By
    pub encoded_by: String,
    /// The actual audio.
    frames: VecDeque<AFrame>,
    /// The total number of frames in the audio.
    n_frames: u32,
}

impl Audio {
    /// Create a new `Audio`.
    pub fn new(n_frames: u32) -> Audio {
        Audio {
            n_frames,
            frames: VecDeque::new(),
            title: String::new(),
            artist: String::new(),
            album_artist: String::new(),
            album: String::new(),
            cd: String::new(),
            release: String::new(),
            track_number: String::new(),
            track_count: String::new(),
            genre: String::new(),
            comment: String::new(),
            composer: String::new(),
            orig_artist: String::new(),
            copyright: String::new(),
            url: String::new(),
            encoded_by: String::new(),
        }
    }

    /// Returns audio for the next frame on the Queue.
    pub fn pop(&mut self) -> Option<AFrame> {
        Some(self.frames.pop_front()?)
    }

    /// Return the number of frames in the audio.
    pub fn frames(&self) -> u32 {
        self.n_frames
    }
}
