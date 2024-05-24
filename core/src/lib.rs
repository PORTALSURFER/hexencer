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
    pub message: MidiMessage,
    pub on: bool,
}

impl MidiEvent {
    pub fn new(tick: u64, midi_message: MidiMessage, on: bool) -> Self {
        Self {
            tick,
            message: midi_message,
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

#[derive(Default)]
pub struct Track {
    pub id: usize,
    pub name: String,
    pub events: Vec<MidiEvent>,
}
impl Track {
    fn new(id: usize, name: &str) -> Track {
        let event = MidiEvent {
            tick: 0,
            message: MidiMessage::NoteOn(
                Note {
                    index: 60,
                    channel: 1,
                    velocity: 100,
                    duration: 1000,
                },
                Instrument {
                    name: String::from("piano"),
                    midi_port: 0,
                },
            ),
            on: true,
        };
        Self {
            id,
            name: String::from(name),
            events: vec![event; 8],
        }
    }
}

pub struct Commander {}

impl Commander {
    pub fn execute(&self, command: &str) {
        match command {
            "get_tracks" => {
                println!("Playing!");
            }
            _ => {
                println!("Unknown command!");
            }
        }
    }
}
