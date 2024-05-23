#[derive(Debug)]
pub struct Instrument {
    pub name: String,
    pub midi_port: u8,
}

#[derive(Debug)]
pub struct MidiEvent {
    pub tick: u64,
    pub message: MidiMessage,
}

#[derive(Debug)]
pub struct Note {
    pub index: u8,
    pub channel: u8,
    pub velocity: u8,
    pub duration: u64,
}

#[derive(Debug)]
pub enum MidiMessage {
    NoteOn(Note, Instrument),
    NoteOff(Note),
}

#[derive(Default)]
pub struct Track {
    pub events: Vec<MidiEvent>,
}

#[derive(Default)]
pub struct TrackManager {
    tracks: Vec<Track>,
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
        let track = Track::default();
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
