pub mod data;
pub mod instrument;
pub mod note;
pub mod trig;

use data::trig_list::{EventBlock, EventList};
use instrument::Instrument;
use note::NoteEvent;
use std::fmt::Display;
use trig::{Event, EventEntry, UniqueId};

#[derive(Default, PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Copy)]
pub struct Tick(u64);

impl Tick {
    pub fn as_beat(&self) -> u32 {
        (self.0 / 480) as u32 + 1
    }

    pub fn tick(&mut self) {
        self.0 = self.0 + 1;
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    fn offset(mut self, length: u32) -> Self {
        self.0 = self.0 + length as u64;
        self
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

#[derive(Default)]
pub struct Track {
    pub id: usize,
    pub name: String,
    pub instrument: Instrument,
    pub event_list: EventList,
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}, instrument: {}", self.name, self.instrument))?;
        Ok(())
    }
}

impl Track {
    fn new(id: usize, name: &str) -> Track {
        let mut event_list = EventList::new();
        for i in 0..8 {
            let event_block = EventBlock::new_midi(Tick::from(i * 480 as u32), 120, 38, 64);
            event_list.add_event_block(event_block);
        }

        Self {
            id,
            name: String::from(name),
            event_list,
            instrument: Instrument::new("port0", 0, 0),
        }
    }

    pub fn set_port(&mut self, port: u8) {
        self.instrument.port = port;
    }
}
