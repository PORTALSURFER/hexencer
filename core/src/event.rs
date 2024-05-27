use crate::data::DataId;
use crate::data::MidiMessage;

use std::fmt::Display;

/// wraps events
#[derive(Debug, Clone)]
pub struct Event {
    id: DataId,
    /// type of this event
    pub inner: EventType,
    /// true if this event is active and should be used
    pub active: bool,
}

impl Event {
    /// creates a new event entry
    pub fn new(id: DataId, event: EventType, active: bool) -> Self {
        Self {
            id,
            inner: event,
            active,
        }
    }
}

/// event type
#[derive(Debug, Clone)]
pub enum EventType {
    /// midi event
    Midi(MidiMessage),
}

impl EventType {
    /// get copy of the midi message in this event
    pub fn get_message(&self) -> MidiMessage {
        match self {
            EventType::Midi(message) => message.clone(),
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

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Midi(message) => f.write_str(&format!("{}", message)),
        }
    }
}

impl EventType {
    /// creates a new midi event
    pub fn new(event: MidiMessage) -> Self {
        Self::Midi(event)
    }
}
