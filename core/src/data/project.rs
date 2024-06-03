use super::{
    clip::Clip,
    track::{Track, TrackCollection},
    InstrumentManager,
};

/// presents a hexencer project
#[derive(Default, Debug)]
pub struct Project {
    /// collection of tracks for this project
    pub tracks: TrackCollection,
    /// collection of instruments for this project
    pub instrument_manager: InstrumentManager,
}

impl Project {
    /// create a new project mananger
    pub fn new() -> Self {
        Self {
            tracks: TrackCollection::default(),
            instrument_manager: InstrumentManager::default(),
        }
    }

    /// get the current track count
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// add a new track to the collection
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    // pub fn get_all_event_entries(&self) -> Vec<(Tick, EventEntry)> {
    // self.track_manager.get_all_event_entries()
    // }

    /// remove a track from the collection
    pub fn remove_track(&mut self) {
        self.tracks.pop();
    }

    /// returns reference to the clip if found, else 'None'
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

    /// add a new track to the end of the collection
    pub fn push_track(&mut self) {
        let index = self.tracks.len();
        let track = Track::new(index, "test");
        self.add_track(track);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_clip_after_adding() {
        let mut project = Project::new();
        let mut track = Track::new(0, "track 0");
        let clip = Clip::new("new_clip", 120);
        let id = clip.get_id();
        track.add_clip(100.into(), clip);

        project.add_track(track);

        for clip in project.find_clip(id) {
            assert!(clip.get_id() == id);
        }
    }
}
