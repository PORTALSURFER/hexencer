use std::fmt::Display;

use super::common::{ALL_NOTE_ON_MSG, NOTE_OFF_MSG, NOTE_ON_MSG};

/// midi message types
#[derive(Debug, Clone, Copy)]
pub enum MidiMessage {
    /// note on midi message
    NoteOn {
        /// the 'key' field represents a 0-127 value which is mapped to a note in the scale
        key: u8,
        /// 'velocity' represents the speed of the note activation
        velocity: u8,
    },
    /// note off midi message
    NoteOff {
        /// the 'key' field represents a 0-127 value which is mapped to a note in the scale
        key: u8,
        /// 'velocity' represents the speed of the note activation
        velocity: u8,
    },
    /// all notes off midi message
    AllNoteOff,
}

impl MidiMessage {
    /// converts this MidiMessage to bits ready to send to a midi port
    pub fn to_midi(&self, channel: u8) -> [u8; 3] {
        match self {
            MidiMessage::NoteOn { key, velocity } => [NOTE_ON_MSG | channel, *key, *velocity],
            MidiMessage::NoteOff { key, velocity } => [NOTE_OFF_MSG | channel, *key, *velocity],
            MidiMessage::AllNoteOff => [ALL_NOTE_ON_MSG, 123, 0],
        }
    }

    pub(crate) fn get_key(&self) -> u8 {
        match self {
            MidiMessage::NoteOn { key, .. } => *key,
            MidiMessage::NoteOff { key, .. } => *key,
            MidiMessage::AllNoteOff => 0,
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
                f.write_str(&format!("[note_on]key:{}, velocity:{}", key, velocity))
            }
            MidiMessage::NoteOff { key, velocity } => {
                f.write_str(&format!("[note_off]key:{}, velocity:{}", key, velocity))
            }
            MidiMessage::AllNoteOff => f.write_str("[global_note_off]"),
        }
    }
}
