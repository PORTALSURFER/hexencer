use crate::{Instrument, MidiEvent, Track};

#[derive(Default)]
pub struct DataLayer {
    pub project_manager: ProjectManager,
}

#[derive(Default)]
pub struct ProjectManager {
    pub track_manager: TrackManager,
    pub instrument_manager: InstrumentManager,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            track_manager: TrackManager::default(),
            instrument_manager: InstrumentManager::default(),
        }
    }
    pub fn track_count(&self) -> usize {
        self.track_manager.tracks.len()
    }

    pub fn add_track(&mut self) {
        let track_count = self.track_manager.tracks.len();
        let track = Track::new(track_count, &format!("track {}", track_count));
        self.track_manager.tracks.push(track);
    }

    pub fn get_all_events(&self) -> Vec<MidiEvent> {
        self.track_manager.get_all_events()
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

    fn get_all_events(&self) -> Vec<MidiEvent> {
        let mut events: Vec<MidiEvent> = self
            .tracks
            .iter()
            .flat_map(|track| track.events.clone())
            .collect();
        events.sort_by_key(|event| event.tick);

        events
    }
}
