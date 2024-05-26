use super::{track::TrackManager, trig_list::EventList, InstrumentManager};
use crate::{event::EventEntry, Tick, Track};

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

    // pub fn get_all_event_entries(&self) -> Vec<(Tick, EventEntry)> {
    // self.track_manager.get_all_event_entries()
    // }

    pub fn remove_track(&mut self) {
        self.track_manager.tracks.pop();
    }
}
