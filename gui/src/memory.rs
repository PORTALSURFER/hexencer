use egui::{Id, Ui};
use hexencer_core::DataId;

/// gui wide shared state, used for storing interaction states used throughout the gui
#[derive(Clone, Default)]
pub struct GuiState {
    /// id of the currently selected clip
    pub selected_clip: Option<DataId>,
}

impl GuiState {
    /// loads the state, or creates a new one if it does not yet exist
    pub fn load(ui: &Ui) -> Self {
        ui.memory(|memory| memory.data.get_temp(Id::NULL))
            .unwrap_or_default()
    }

    /// overwrite state with new state
    pub fn store(self, ui: &Ui) {
        ui.memory_mut(|memory| memory.data.insert_temp(Id::NULL, self));
    }
}
