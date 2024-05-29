use super::{
    clip::Clip,
    track::{Track, Tracks},
    InstrumentManager,
};

#[derive(Default)]
pub struct ProjectManager {
    pub tracks: Tracks,
    pub instrument_manager: InstrumentManager,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            tracks: Tracks::default(),
            instrument_manager: InstrumentManager::default(),
        }
    }
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    pub fn add_track(&mut self) {
        let track_count = self.tracks.len();
        let track = Track::new(track_count, &format!("track {}", track_count));
        self.tracks.push(track);
    }

    // pub fn get_all_event_entries(&self) -> Vec<(Tick, EventEntry)> {
    // self.track_manager.get_all_event_entries()
    // }

    pub fn remove_track(&mut self) {
        self.tracks.pop();
    }

    pub fn find_clip(&self, selected_clip_id: crate::DataId) -> Option<&Clip> {
        for track in self.tracks.iter() {
            for (_, clip) in track.clips.iter() {
                if clip.get_id() == selected_clip_id {
                    let clip = clip.to_owned();
                    return Some(clip);
                }
            }
        }
        None
    }
}
