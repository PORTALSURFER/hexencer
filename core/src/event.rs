use crate::data::DataId;
use crate::data::MidiMessage;

use std::fmt::Display;
use std::ops::Deref;

/// wraps events
#[derive(Debug, Clone, Copy)]
pub struct Event {
    /// id of this event
    id: EventId,
    /// type of this event
    pub inner: EventType,
    /// true if this event is active and should be used
    pub active: bool,
}

impl Event {
    /// creates a new event entry
    pub fn new(id: EventId, event: EventType, active: bool) -> Self {
        Self {
            id,
            inner: event,
            active,
        }
    }

    /// returns the key of this event or 0 if none is found
    pub fn get_key(&self) -> u8 {
        match self.inner {
            EventType::Midi(midi_message) => midi_message.get_key(),
        }
    }

    /// get the end of this note event, TODO conflic with EventSegment which also has note end
    pub fn get_note_end(&self) -> f32 {
        96.0
    }
}

/// event type
#[derive(Debug, Clone, Copy)]
pub enum EventType {
    /// midi event
    Midi(MidiMessage),
}

impl EventType {
    /// get copy of the midi message in this event
    pub fn get_message(&self) -> MidiMessage {
        match self {
            EventType::Midi(message) => *message,
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

/// data identifier of a clip
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventId(DataId);

impl Deref for EventId {
    type Target = DataId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl EventId {
    /// creates a new clip id
    pub(crate) fn new() -> Self {
        Self(DataId::new())
    }
}
