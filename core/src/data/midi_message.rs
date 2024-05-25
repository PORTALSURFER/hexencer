use std::fmt::Display;

use crate::note::Note;

use super::{ALL_NOTE_ON_MSG, NOTE_OFF_MSG, NOTE_ON_MSG};

#[derive(Debug, Clone)]
pub enum MidiMessage {
    NoteOn(Note),
    NoteOff(Note),
    GlobalNoteOff,
}

impl MidiMessage {
    pub fn to_midi(&self) -> [u8; 3] {
        match self {
            MidiMessage::NoteOn(note) => [NOTE_ON_MSG, note.index, note.velocity],
            MidiMessage::NoteOff(note) => [NOTE_OFF_MSG, note.index, note.velocity],
            MidiMessage::GlobalNoteOff => [ALL_NOTE_ON_MSG, 123, 0],
        }
    }
}

impl Default for MidiMessage {
    fn default() -> Self {
        Self::NoteOn(Note::new(66, 0, 64))
    }
}

impl Display for MidiMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiMessage::NoteOn(note) => f.write_str(&format!("[note_on]{}", note)),
            MidiMessage::NoteOff(note) => f.write_str(&format!("[note_off]{}", note)),
            MidiMessage::GlobalNoteOff => f.write_str(&format!("[global_note_off]")),
        }
    }
}
