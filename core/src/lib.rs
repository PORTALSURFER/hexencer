use std::fmt::Display;

pub mod data;

#[derive(Debug, Clone, Default)]
pub struct Instrument {
    pub name: String,
    pub midi_port: u8,
}

impl Instrument {
    pub fn new(name: &str, midi_port: u8) -> Self {
        Self {
            name: String::from(name),
            midi_port,
        }
    }
}

impl Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "[instrument|name:{}, midi_port:{}]",
            self.name, self.midi_port
        ))
    }
}

#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub tick: u64,
    pub midi_message: MidiMessage,
    pub instrument: Instrument,
    pub duration: u32,
    pub on: bool,
}

const NOTE_ON_MSG: u8 = 0x90;
const ALL_NOTE_ON_MSG: u8 = 0xB0;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY: u8 = 0x64;

impl MidiEvent {
    pub fn to_midi(&self) -> Vec<u8> {
        let (note, status) = match &self.midi_message {
            MidiMessage::NoteOn(note) => (Some(note), NOTE_ON_MSG),
            MidiMessage::NoteOff(note) => (Some(note), NOTE_OFF_MSG),
            MidiMessage::GlobalNoteOff => (None, ALL_NOTE_ON_MSG),
        };

        let (data_c, data_v) = note
            .map(|note| (note.index, note.velocity))
            .unwrap_or((120, 0));
        vec![status, data_c, data_v]
    }

    pub fn get_note_index(&self) -> u8 {
        match &self.midi_message {
            MidiMessage::NoteOn(note) => note.index,
            _ => 0,
        }
    }

    pub fn global_note_off(instrument: Instrument) -> Self {
        Self {
            tick: 0,
            midi_message: MidiMessage::GlobalNoteOff,
            instrument,
            on: false,
            duration: 0,
        }
    }
}

impl Display for MidiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "beat:{}, message:{}, instrument:{}",
                self.tick / 480,
                self.midi_message,
                self.instrument
            )
            .as_str(),
        )
    }
}

impl MidiEvent {
    pub fn new(tick: u64, midi_message: MidiMessage, on: bool) -> Self {
        Self {
            tick,
            midi_message,
            instrument: Instrument::default(),
            on,
            duration: 480,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub index: u8,
    pub channel: u8,
    pub velocity: u8,
}

impl Note {
    pub fn new(index: u8, channel: u8, velocity: u8) -> Self {
        Self {
            index,
            channel,
            velocity,
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "[note|index: {}, channel: {}, velocity: {}",
                self.index, self.channel, self.velocity
            )
            .as_str(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum MidiMessage {
    NoteOn(Note),
    NoteOff(Note),
    GlobalNoteOff,
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

#[derive(Default)]
pub struct Track {
    pub id: usize,
    pub name: String,
    pub events: Vec<MidiEvent>,
    pub instrument: Instrument,
}
impl Track {
    fn new(id: usize, name: &str, channel: u8) -> Track {
        let mut events = Vec::new();

        for i in 0..=8 {
            let event = MidiEvent {
                tick: i * 480,
                midi_message: MidiMessage::NoteOn(Note {
                    index: 39,
                    channel: 0,
                    velocity: 127,
                }),
                on: true,
                instrument: Instrument::default(),
                duration: 100,
            };
            events.push(event);
        }
        Self {
            id,
            name: String::from(name),
            events,
            instrument: Instrument::new("port0", 0),
        }
    }

    pub fn set_port(&mut self, port: u8) {
        self.instrument.midi_port = port;
        for event in self.events.iter_mut() {
            event.instrument.midi_port = port;
        }
    }
}
