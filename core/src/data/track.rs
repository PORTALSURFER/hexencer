use std::collections::BTreeMap;

use crate::{trig::Trig, Tick, Track};

#[derive(Default)]
pub struct TrackManager {
    pub tracks: Vec<Track>,
}

impl TrackManager {
    pub fn add(&mut self, new_track: Track) {
        self.tracks.push(new_track);
    }

    pub fn get_all_trigs(&self) -> Vec<&BTreeMap<Tick, Trig>> {
        self.tracks.iter().map(|track| &track.trigs.0).collect()
    }
}
