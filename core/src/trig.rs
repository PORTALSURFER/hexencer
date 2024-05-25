use crate::{data::midi_message::MidiMessage, NoteEvent};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Trig {
    pub on: bool,
    pub event: NoteEvent,
}

impl Trig {
    pub fn get_key(&self) -> u8 {
        self.event.key
    }

    pub fn get_velocity(&self) -> u8 {
        self.event.velocity
    }

    pub fn get_message(&self) -> MidiMessage {
        match self.on {
            true => self.get_note_on(),
            false => self.get_note_off(),
        }
    }

    fn get_note_on(&self) -> MidiMessage {
        MidiMessage::NoteOn {
            key: self.get_key(),
            velocity: self.event.velocity,
        }
    }

    fn get_note_off(&self) -> MidiMessage {
        MidiMessage::NoteOff {
            key: self.get_key(),
            velocity: self.get_velocity(),
        }
    }
}

impl Display for Trig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("event:{}, ", self.event))
    }
}

impl Trig {
    pub fn new(event: NoteEvent) -> Self {
        Self { event, on: true }
    }
}
