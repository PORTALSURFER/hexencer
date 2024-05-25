use crate::{trig::Trig, Tick, Track};

#[derive(Default)]
pub struct TrackManager {
    pub tracks: Vec<Track>,
}

impl TrackManager {
    pub fn add(&mut self, new_track: Track) {
        self.tracks.push(new_track);
    }

    pub fn get_all_trigs(&self) -> Vec<(Tick, Trig)> {
        self.tracks
            .iter()
            .flat_map(|track| track.trigs.iter().map(|(&tick, trig)| (tick, trig.clone())))
            .collect()
    }
}
