mod clip;
mod common;
mod midi_event;
mod midi_message;
mod project;
mod track;

pub use common::DataId;
pub use midi_message::MidiMessage;

/// event list
pub mod event_list;

use self::project::ProjectManager;
use crate::{instrument::Instrument, Tick};

/// holds state of the editor, like note editor or automation editor modes.
#[derive(Default)]
pub struct EditorState {
    selected_clip: DataId,
}

/// object which holds all the persistent data objects used by the application
#[derive(Default)]
pub struct DataLayer {
    /// interface for loading and storing projects
    pub project_manager: ProjectManager,
    /// represents the current state of the editor, like note editor or automation editor modes.
    pub editor_state: EditorState,
    tick: Tick,
}

impl DataLayer {
    /// add a new clip to the track specified by 'track_id'
    pub fn add_clip(&mut self, track_id: usize, tick: Tick, name: &str) {
        self.project_manager
            .tracks
            .get_mut(track_id)
            .unwrap()
            .add_clip(tick, name);
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
#[derive(Default)]
pub struct InstrumentManager {
    inner: Vec<Instrument>,
}
