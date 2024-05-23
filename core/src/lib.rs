#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: String,
    pub midi_port: u8,
}

#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub tick: u64,
    pub message: MidiMessage,
    pub on: bool,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub index: u8,
    pub channel: u8,
    pub velocity: u8,
    pub duration: u64,
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

#[derive(Default)]
pub struct TrackManager {
    pub tracks: Vec<Track>,
}

#[derive(Default)]
pub struct InstrumentManager {
    pub instruments: Vec<Instrument>,
}

impl TrackManager {
    pub fn add(&mut self, new_track: Track) {
        self.tracks.push(new_track);
    }
}

#[derive(Default)]
pub struct ProjectManager {
    pub track_manager: TrackManager,
    pub instrument_manager: InstrumentManager,
}
impl ProjectManager {
    pub fn track_count(&self) -> usize {
        self.track_manager.tracks.len()
    }

    pub fn add_track(&mut self) {
        let track_count = self.track_manager.tracks.len();
        let track = Track::new(track_count, &format!("track {}", track_count));
        self.track_manager.tracks.push(track);
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
