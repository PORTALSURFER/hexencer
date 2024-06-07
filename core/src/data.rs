/// clip data object
mod clip;
/// common objects
mod common;
/// the midi event objects
mod midi_event;
/// the midi message object
mod midi_message;
/// the project data object
mod project;
/// the track data object
mod track;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub use clip::Clip;
pub use clip::ClipId;
pub use common::DataId;
pub use midi_message::MidiMessage;
pub use track::TrackId;
/// event list
pub mod event_list;

use self::project::Project;
use crate::{instrument::Instrument, Tick};
use thiserror::Error;

/// holds state of the editor, like note editor or automation editor modes.
#[derive(Default, Debug)]
pub struct EditorState {
    /// data id of the selected clip, so it can be found later
    selected_clip: DataId,
}

/// interface for talking with main hexencer data object
#[derive(Default, Debug, Clone)]
pub struct DataInterface {
    /// inner object actually holding data
    inner: Arc<std::sync::Mutex<DataLayer>>,
}

impl DataInterface {
    /// get funtion to lock and get data
    pub fn get(&self) -> MutexGuard<DataLayer> {
        self.inner.lock().expect("illegal lock")
    }

    /// creates a new interface for data
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(DataLayer::default())),
        }
    }
}
/// error type for data_layer
#[derive(Error, Debug)]
pub enum DataLayerError {
    /// when an action on a track is attempted, but no track with that id exists
    #[error("No track with id {0}")]
    NoTrack(TrackId),
}

/// object which holds all the persistent data objects used by the application
#[derive(Default, Debug)]
pub struct DataLayer {
    /// interface for loading and storing projects
    pub project_manager: Project,
    /// represents the current state of the editor, like note editor or automation editor modes.
    pub editor_state: EditorState,
    /// current tick passed to data layer to give the gui access to it, originally in the sequencer
    /// TODO needs cleanup
    tick: Tick,
}

impl DataLayer {
    /// add a new clip to the track specified by 'track_id'
    pub fn add_clip(
        &mut self,
        track_id: TrackId,
        tick: Tick,
        clip: Clip,
    ) -> Result<(), DataLayerError> {
        if let Some(track) = self.project_manager.tracks.get_mut(track_id) {
            track.add_clip(tick, clip);
        } else {
            return Err(DataLayerError::NoTrack(track_id));
        }
        Ok(())
    }

    /// set the playhead tick
    pub fn set_tick(&mut self, tick: Tick) {
        self.tick = tick;
    }

    /// get the current playhead tick
    pub fn get_tick(&self) -> Tick {
        self.tick
    }
}

/// keeps track and manages all instruments
#[derive(Default, Debug)]
pub struct InstrumentManager {
    /// inner list of instruments managed
    inner: Vec<Instrument>,
}

#[cfg(test)]
mod tests {
    use self::track::{Track, TrackId};

    use super::*;
    use coverage_helper::test;

    #[test]
    fn test_add_clip() {
        let mut data = DataLayer::default();
        let track = Track::new(TrackId::new(), "test");
        data.project_manager.add_track(track);

        {
            let clips = data.project_manager.tracks.get_clips(0).unwrap();
            assert!(clips.len() == 0);
        }
        let clip = Clip::new("test", 120);
        data.add_clip(TrackId::new(), Tick::from(0), clip)
            .expect("failed to add clip"); //TODO this is flawed, adding to some unkown id

        {
            let clips = data.project_manager.tracks.get_clips(0).unwrap();
            assert!(clips.len() > 0);
        }
    }

    #[test]
    fn sets_and_gets_tick() {
        let mut data = DataLayer::default();
        data.set_tick(Tick::from(100));
        assert_eq!(data.get_tick(), Tick::from(100));
        data.set_tick(Tick::from(999));
        assert_eq!(data.get_tick(), Tick::from(999));
    }
}
