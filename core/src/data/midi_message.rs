use super::{ALL_NOTE_ON_MSG, NOTE_OFF_MSG, NOTE_ON_MSG};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum MidiMessage {
    NoteOn { key: u8, velocity: u8 },
    NoteOff { key: u8, velocity: u8 },
    GlobalNoteOff,
}

impl MidiMessage {
    pub fn to_midi(&self) -> [u8; 3] {
        match self {
            MidiMessage::NoteOn { key, velocity } => [NOTE_ON_MSG, *key, *velocity],
            MidiMessage::NoteOff { key, velocity } => [NOTE_OFF_MSG, *key, *velocity],
            MidiMessage::GlobalNoteOff => [ALL_NOTE_ON_MSG, 123, 0],
        }
    }
}

impl Default for MidiMessage {
    fn default() -> Self {
        Self::NoteOn {
            key: 66,
            velocity: 64,
        }
    }
}

impl Display for MidiMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiMessage::NoteOn { key, velocity } => {
                f.write_str(&format!("[note_on]{} {}", key, velocity))
            }
            MidiMessage::NoteOff { key, velocity } => {
                f.write_str(&format!("[note_off]{} {}", key, velocity))
            }
            MidiMessage::GlobalNoteOff => f.write_str(&format!("[global_note_off]")),
        }
    }
}
