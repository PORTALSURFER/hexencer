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

pub use clip::Clip;
pub use common::DataId;
pub use midi_message::MidiMessage;

/// event list
pub mod event_list;

use self::project::Project;
use crate::{instrument::Instrument, Tick};

/// holds state of the editor, like note editor or automation editor modes.
#[derive(Default, Debug)]
pub struct EditorState {
    /// data id of the selected clip, so it can be found later
    selected_clip: DataId,
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
    pub fn add_clip(&mut self, track_id: usize, tick: Tick, name: &str, end: u64) {
        self.project_manager
            .tracks
            .get_mut(track_id)
            .unwrap()
            .add_clip(tick, name, end);
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
