use crate::data::Id;
use crate::data::MidiMessage;

use std::fmt::Display;

/// wraps events
#[deprecated(note = "change code to use event directly")]
#[derive(Debug, Clone)]
pub struct EventEntry {
    id: Id,
    /// event this entry wraps
    pub event: Event,
    /// true if this event is active and should be used
    pub active: bool,
}

impl EventEntry {
    /// creates a new event entry
    pub fn new(id: Id, event: Event, active: bool) -> Self {
        Self { id, event, active }
    }
}

/// event type
#[derive(Debug, Clone)]
pub enum Event {
    /// midi event
    Midi(MidiMessage),
}

impl Event {
    /// get copy of the midi message in this event
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
    /// creates a new midi event
    pub fn new(event: MidiMessage) -> Self {
        Self::Midi(event)
    }
}
