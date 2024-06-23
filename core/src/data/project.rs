use crate::Tick;

use super::{
    clip::{Clip, ClipId},
    track::{Track, TrackCollection, TrackId},
    InstrumentManager,
};

/// presents a hexencer project
#[derive(Default, Debug)]
pub struct Project {
    /// collection of tracks for this project
    pub track_collection: TrackCollection,
    /// collection of instruments for this project
    pub instrument_manager: InstrumentManager,
}

impl Project {
    /// create a new project mananger
    pub fn new() -> Self {
        Self {
            track_collection: TrackCollection::default(),
            instrument_manager: InstrumentManager::default(),
        }
    }

    /// get the current track count
    pub fn track_count(&self) -> usize {
        self.track_collection.len()
    }

    /// add a new track to the collection
    pub fn add_track(&mut self, track: Track) {
        self.track_collection.push(track);
    }

    // pub fn get_all_event_entries(&self) -> Vec<(Tick, EventEntry)> {
    // self.track_manager.get_all_event_entries()
    // }

    /// remove a track from the collection
    pub fn remove_track(&mut self) {
        self.track_collection.pop();
    }

    /// returns reference to the clip if found, else 'None'
    pub fn find_clip(&self, target_clip_id: ClipId) -> Option<Clip> {
        for track in self.track_collection.iter() {
            for (clip_key, clip) in track.clips.iter() {
                if clip_key.id == target_clip_id {
                    let clip = clip.to_owned();
                    return Some(clip);
                }
            }
        }
        None
    }

    /// add a new track to the end of the collection
    pub fn push_track(&mut self) {
        let track = Track::new(TrackId::new(), "test");
        self.add_track(track);
    }

    /// moved a clip from one track to another
    pub fn move_clip(&mut self, clip_id: ClipId, track_id: TrackId, tick: Tick) {
        let _ = track_id;
        let clip = self.track_collection.take_clip(clip_id);
        if let Some(mut clip) = clip {
            clip.start = tick;
            if let Some(track) = self.track_collection.get_mut(track_id) {
                track.add_clip(clip);
            }
        } else {
            tracing::info!("clip was not found {}", clip_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Tick;

    use super::*;

    #[test]
    fn can_find_clip_after_adding() {
        let mut project = Project::new();
        let mut track = Track::new(TrackId::new(), "track 0");
        let clip = Clip::new(Tick::from(120), "new_clip", Tick::from(120));
        track.add_clip(clip);

        project.add_track(track);

        // assert!(clip.id() == clip_id);
    }
}
