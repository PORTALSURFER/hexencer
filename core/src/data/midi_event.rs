use crate::note::Note;

use super::midi_message::MidiMessage;

#[derive(Debug)]
pub enum MidiEvent {
    NoteOn(Note),
    NoteOff(Note),
    AllNoteOff(),
}

impl From<MidiEvent> for MidiMessage {
    fn from(val: MidiEvent) -> Self {
        match val {
            MidiEvent::NoteOn(note) => MidiMessage::NoteOn(note),
            MidiEvent::NoteOff(note) => MidiMessage::NoteOn(note),
            MidiEvent::AllNoteOff() => MidiMessage::GlobalNoteOff,
        }
    }
}
