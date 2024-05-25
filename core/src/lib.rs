pub mod data;
pub mod instrument;
pub mod note;
pub mod trig;

use std::{collections::BTreeMap, fmt::Display};

use data::{event_list::EventList, ALL_NOTE_ON_MSG, NOTE_OFF_MSG, NOTE_ON_MSG};
use instrument::Instrument;
use note::Note;
use trig::Trig;

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

#[derive(Default)]
pub struct Trigs(pub BTreeMap<Tick, Trig>);

impl Trigs {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Tick, &Trig)> {
        self.0.iter()
    }

    pub fn build_event_list(&self) -> EventList {
        let mut event_list = EventList::new();
        for (tick, trig) in self.iter() {
            match trig.on {
                true => {
                    event_list.insert(*tick, trig.get_note_on());
                }
                false => {
                    event_list.insert(*tick, trig.get_note_off());
                }
            }
        }
        event_list
    }
}

#[derive(Default)]
pub struct Track {
    pub id: usize,
    pub name: String,
    pub trigs: Trigs,
    pub instrument: Instrument,
    pub event_list: EventList,
}
impl Track {
    fn new(id: usize, name: &str, channel: u8) -> Track {
        let mut trigs = Trigs::new();

        for i in 0..8 {
            let trig = Trig {
                note: Note {
                    index: 39,
                    channel: 0,
                    velocity: 127,
                },
                on: true,
                instrument: Instrument::default(),
                duration: 100,
            };
            trigs.0.insert(Tick::from(i * 480 as usize), trig);
        }

        let event_list = trigs.build_event_list();

        Self {
            id,
            name: String::from(name),
            trigs,
            instrument: Instrument::new("port0", 0, 0),
            event_list,
        }
    }

    pub fn set_port(&mut self, port: u8) {
        self.instrument.midi_port = port;
        for (_, trig) in self.trigs.0.iter_mut() {
            trig.instrument.midi_port = port;
        }
    }
}
