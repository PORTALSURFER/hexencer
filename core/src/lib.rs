#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]

//! represents elements without clear goal

/// houses common data types
pub mod data;
/// houses event types
pub mod event;
/// instrument types
pub mod instrument;

pub use data::DataId;

use std::fmt::Display;
use std::time::Duration;

/// represents a moment in time
/// events are sent every tick
#[derive(Default, PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Copy)]
pub struct Tick(u64);

impl Tick {
    /// convert tick to a time string
    pub fn as_time(&self) -> String {
        let bpm = 120.0;
        let ppqn = 480.0;
        let seconds_in_beat = 60.0 / bpm;
        let seconds_in_tick = seconds_in_beat / ppqn;

        let total_seconds = self.0 as f64 * seconds_in_tick;

        let duration = Duration::from_secs_f64(total_seconds);

        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        let milliseconds = duration.subsec_millis();

        format!("{}:{}:{}", minutes as u32, seconds as u32, milliseconds,)
    }

    /// turns this 'Tick' into a more human readable beat
    pub fn as_beat(&self) -> u32 {
        (self.0 / 480) as u32 + 1
    }

    /// move the tick one step forward in time
    pub fn tick(&mut self) {
        self.0 += 1;
    }

    /// created a zero tick
    pub fn zero() -> Self {
        Self(0)
    }

    /// reset the tick to 0
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// adds an offset to this 'Tick'
    fn offset(mut self, length: u32) -> Self {
        self.0 += length as u64;
        self
    }

    /// returns this 'Tick' as an 'f32'
    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

impl Display for Tick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.0))
    }
}

impl From<i32> for Tick {
    fn from(tick: i32) -> Self {
        Self(tick as u64)
    }
}

impl From<f32> for Tick {
    fn from(tick: f32) -> Self {
        Self(tick as u64)
    }
}

impl From<usize> for Tick {
    fn from(tick: usize) -> Self {
        Self(tick as u64)
    }
}

impl From<u64> for Tick {
    fn from(tick: u64) -> Self {
        Self(tick)
    }
}

impl From<u32> for Tick {
    fn from(tick: u32) -> Self {
        Self(tick as u64)
    }
}
