use std::fmt::Display;

use crate::{data::midi_event::MidiEvent, instrument::Instrument, Note, Tick};

#[derive(Debug, Clone)]
pub struct Trig {
    pub instrument: Instrument,
    pub duration: u32,
    pub on: bool,
    pub note: Note,
}

impl Trig {
    // pub fn to_midi(&self) -> Vec<u8> {
    //     let (note, status) = match &self.midi_message {
    //         MidiMessage::NoteOn(note) => (Some(note), NOTE_ON_MSG),
    //         MidiMessage::NoteOff(note) => (Some(note), NOTE_OFF_MSG),
    //         MidiMessage::GlobalNoteOff => (None, ALL_NOTE_ON_MSG),
    //     };

    //     let (data_c, data_v) = note
    //         .map(|note| (note.index, note.velocity))
    //         .unwrap_or((120, 0));
    //     vec![status, data_c, data_v]
    // }

    pub fn get_note_index(&self) -> u8 {
        self.note.index
    }

    pub fn get_note_on(&self) -> crate::data::midi_event::MidiEvent {
        MidiEvent::NoteOn(self.note)
    }

    pub fn get_note_off(&self) -> crate::data::midi_event::MidiEvent {
        MidiEvent::NoteOff(self.note)
    }
}

impl Display for Trig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "note:{}, instrument:{}",
            self.note, self.instrument
        ))
    }
}

impl Trig {
    pub fn new(instrument: Instrument, duration: u8, on: bool) -> Self {
        Self {
            instrument,
            on,
            duration: 480,
            note: Note::default(),
        }
    }
}
