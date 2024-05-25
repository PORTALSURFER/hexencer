pub mod event_list;
pub mod midi_event;
pub mod midi_message;
mod project;
mod track;

use self::project::ProjectManager;
use crate::instrument::Instrument;

pub const NOTE_ON_MSG: u8 = 0x90;
pub const ALL_NOTE_ON_MSG: u8 = 0xB0;
pub const NOTE_OFF_MSG: u8 = 0x80;
pub const VELOCITY: u8 = 0x64;

#[derive(Default)]
pub struct DataLayer {
    pub project_manager: ProjectManager,
}

#[derive(Default)]
pub struct InstrumentManager {
    pub instruments: Vec<Instrument>,
}
