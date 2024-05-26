use crate::{
    event::{Event, EventEntry},
    Tick, Track,
};

#[derive(Default)]
pub struct TrackManager {
    pub tracks: Vec<Track>,
}

impl TrackManager {
    pub fn add(&mut self, new_track: Track) {
        self.tracks.push(new_track);
    }

    pub fn get_track_port(&self, index: usize) -> u8 {
        self.tracks.get(index).unwrap().instrument.port
    }

    // pub fn get_all_event_entries(&self) -> Vec<(Tick, EventEntry)> {
    //     self.tracks
    //         .iter()
    //         .flat_map(|track| {
    //             track
    //                 .event_list
    //                 .iter()
    //                 .map(|(&tick, event_entry)| (tick, event_entry))
    //         })
    //         .collect()
    // }
}
