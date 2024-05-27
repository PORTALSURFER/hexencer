#![deny(missing_docs)]

//! represents elements without clear goal

/// houses common data types
pub mod data;

/// houses event types
pub mod event;

/// instrument types
pub mod instrument;

/// note types
pub mod note;

use data::event_list::{EventBlock, EventList};
use instrument::Instrument;
use std::fmt::Display;

/// represents a moment in tim
/// events are sent every tick
#[derive(Default, PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Copy)]
pub struct Tick(u64);

impl Tick {
    /// turns this 'Tick' into a more human readable beat
    pub fn as_beat(&self) -> u32 {
        (self.0 / 480) as u32 + 1
    }

    /// move the tick one step forward in time
    pub fn tick(&mut self) {
        self.0 = self.0 + 1;
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
        self.0 = self.0 + length as u64;
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
