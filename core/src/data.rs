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

use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

pub use clip::Clip;
pub use clip::ClipId;
pub use common::DataId;
pub use midi_message::MidiMessage;
pub use track::Track;
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
pub struct StorageInterface {
    /// inner object actually holding data
    inner: Arc<std::sync::RwLock<DataLayer>>,
}

impl Deref for StorageInterface {
    type Target = Arc<RwLock<DataLayer>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl StorageInterface {
    /// creates a new interface for data
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(DataLayer::fake_data())),
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
    tick: Tick,
}

impl DataLayer {
    /// add a new clip to the track specified by 'track_id'
    pub fn add_clip(&mut self, track_id: TrackId, clip: Clip) -> Result<(), DataLayerError> {
        if let Some(track) = self.project_manager.tracks.get_mut(track_id) {
            track.add_clip(clip);
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

    fn fake_data() -> DataLayer {
        let mut project_manager = Project::default();

        let track_0 = Track::new(TrackId::new(), "track_0");
        project_manager.add_track(track_0);

        let mut track_1 = Track::new(TrackId::new(), "track_1");
        track_1.add_clip(clip::Clip::new(0.into(), "clip_0", 240.into()));
        project_manager.add_track(track_1);

        let mut track_2 = Track::new(TrackId::new(), "track_2");
        track_2.add_clip(clip::Clip::new(120.into(), "clip_1", 10.into()));
        track_2.add_clip(clip::Clip::new(480.into(), "clip_2", 120.into()));
        project_manager.add_track(track_2);

        let mut track_3 = Track::new(TrackId::new(), "track_2");
        track_3.add_clip(clip::Clip::new(0.into(), "clip_3", 10.into()));
        track_3.add_clip(clip::Clip::new(120.into(), "clip_4", 20.into()));
        track_3.add_clip(clip::Clip::new(240.into(), "clip_5", 420.into()));
        project_manager.add_track(track_3);

        Self {
            project_manager,
            editor_state: EditorState::default(),
            tick: Tick::zero(),
        }
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
    #[should_panic]
    fn cant_add_when_track_not_found() {
        let mut data = DataLayer::default();
        let track = Track::new(TrackId::new(), "test");
        data.project_manager.add_track(track);

        let clip = Clip::new(0.into(), "test", 120.into());
        data.add_clip(TrackId::new(), clip).unwrap();
    }

    #[test]
    fn can_add_track() {
        let mut data = DataLayer::default();
        let track_id = TrackId::new();
        let track = Track::new(track_id, "test");
        data.project_manager.add_track(track);

        {
            let clips = data.project_manager.tracks.get_clips(0).unwrap();
            assert_eq!(clips.len(), 0);
        }

        let clip = Clip::new(0.into(), "test", 120.into());
        data.add_clip(track_id, clip).unwrap();

        {
            let clips = data.project_manager.tracks.get_clips(0).unwrap();
            assert_eq!(clips.len(), 1);
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
