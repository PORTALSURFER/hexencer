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
use crate::instrument::Instrument;

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
}

/// keeps track and manages all instruments
#[derive(Default)]
pub struct InstrumentManager {
    inner: Vec<Instrument>,
}
