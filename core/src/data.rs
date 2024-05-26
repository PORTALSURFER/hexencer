pub mod common;
pub mod midi_event;
pub mod midi_message;
mod project;
mod track;
pub mod trig_list;

use self::project::ProjectManager;
use crate::instrument::Instrument;

#[derive(Default)]
pub struct DataLayer {
    pub project_manager: ProjectManager,
}

#[derive(Default)]
pub struct InstrumentManager {
    pub instruments: Vec<Instrument>,
}
