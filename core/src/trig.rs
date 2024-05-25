use crate::{data::midi_message::MidiMessage, NoteEvent};
use std::{
    fmt::Display,
    sync::atomic::{AtomicU64, Ordering},
};

static UNIQUE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct UniqueId(u64);

impl UniqueId {
    pub fn new() -> Self {
        Self(UNIQUE_ID.fetch_add(1, Ordering::SeqCst))
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct EventEntry {
    pub id: UniqueId,
    pub event: Event,
    pub active: bool,
}
impl EventEntry {
    pub fn new(id: UniqueId, event: Event, active: bool) -> Self {
        Self { id, event, active }
    }
}

/// event type
#[derive(Debug, Clone)]
pub enum Event {
    Midi(MidiMessage),
}

impl Event {
    pub fn get_message(&self) -> MidiMessage {
        match self {
            Event::Midi(message) => message.clone(),
        }
    }
    // pub fn get_key(&self) -> u8 {
    //     self.event.key
    // }

    // pub fn get_velocity(&self) -> u8 {
    //     self.event.velocity
    // }

    // pub fn get_message(&self) -> MidiMessage {

    // }

    // fn build_note_on_message(&self) -> MidiMessage {
    //     self.note_on()
    // }

    // fn build_note_off_message(&self) -> MidiMessage {
    //     self.note_off()
    // }

    // fn note_on(&self) -> MidiMessage {
    //     MidiMessage::NoteOn {
    //         key: self.get_key(),
    //         velocity: self.get_velocity(),
    //     }
    // }

    // fn note_off(&self) -> MidiMessage {
    //     MidiMessage::NoteOff {
    //         key: self.get_key(),
    //         velocity: self.get_velocity(),
    //     }
    // }

    // pub(crate) fn as_off_trig(&self) -> Event {
    //     let mut event = self.event.clone();
    //     event.on = false;
    //     Event::new(self.length, self.event, true)
    // }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Midi(message) => f.write_str(&format!("{}", message)),
        }
    }
}

impl Event {
    pub fn new(event: MidiMessage) -> Self {
        Self::Midi(event)
    }
}
