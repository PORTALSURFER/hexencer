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

#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub tick: u64,
    pub midi_message: MidiMessage,
    pub instrument: Instrument,
    pub on: bool,
}

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY: u8 = 0x64;

impl MidiEvent {
    pub fn to_midi(&self) -> Vec<u8> {
        let (note, note_message) = match &self.midi_message {
            MidiMessage::NoteOn(note, channel) => (note, NOTE_ON_MSG),
            MidiMessage::NoteOff(note) => (note, NOTE_OFF_MSG),
        };
        let velocity = note.velocity;
        let index = note.index;
        vec![note_message, index, velocity]
    }
}

impl Display for MidiEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "tick {}, message {}, on: {}",
                self.tick, self.midi_message, self.on
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub index: u8,
    pub channel: u8,
    pub velocity: u8,
    pub duration: u64,
}

impl Note {
    pub fn new(index: u8, channel: u8, velocity: u8, duration: u64) -> Self {
        Self {
            index,
            channel,
            velocity,
            duration,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MidiMessage {
    NoteOn(Note, Instrument),
    NoteOff(Note),
}

impl Default for MidiMessage {
    fn default() -> Self {
        Self::NoteOn(
            Note::new(66, 0, 64, 100),
            Instrument::new("default instrument", 0),
        )
    }
}

impl Display for MidiMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiMessage::NoteOn(_, _) => f.write_str("note on"),
            MidiMessage::NoteOff(_) => f.write_str("note off"),
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
                midi_message: MidiMessage::NoteOn(
                    Note {
                        index: 66,
                        channel,
                        velocity: 64,
                        duration: 10,
                    },
                    Instrument {
                        name: String::from("piano"),
                        midi_port: 0,
                    },
                ),
                on: true,
                instrument: Instrument::default(),
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
