use std::fmt::Display;

pub mod data;

#[derive(Debug, Clone)]
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
    pub on: bool,
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
            };
            events.push(event);
        }
        Self {
            id,
            name: String::from(name),
            events,
        }
    }
}
