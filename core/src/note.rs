use crate::data::MidiMessage;
use std::fmt::Display;

// #[derive(Debug, Clone, Copy)]
// pub struct NoteEvent {
//     pub on: bool,
//     pub key: u8,
//     pub velocity: u8,
// }

// impl Default for NoteEvent {
//     fn default() -> Self {
//         Self {
//             on: true,
//             key: 48,
//             velocity: 96,
//         }
//     }
// }

// impl NoteEvent {
//     pub fn new(key: u8, velocity: u8, on: bool) -> Self {
//         Self { key, velocity, on }
//     }

//     pub fn note_on(&self) -> MidiMessage {
//         MidiMessage::NoteOn {
//             key: self.key,
//             channel: 0,
//             velocity: self.velocity,
//         }
//     }

//     pub fn note_off(&self) -> MidiMessage {
//         MidiMessage::NoteOff {
//             key: self.key,
//             channel: 0,
//             velocity: self.velocity,
//         }
//     }
// }

// impl Display for NoteEvent {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(format!("key: {}, velocity: {}", self.key, self.velocity).as_str())
//     }
// }
