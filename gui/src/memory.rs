use egui::{Id, Ui};
use hexencer_core::DataId;

#[derive(Clone, Default)]
pub struct GuiState {
    pub selected_clip: Option<DataId>,
}

impl GuiState {
    pub fn load(ui: &Ui) -> Self {
        ui.memory(|memory| memory.data.get_temp(Id::NULL))
            .unwrap_or_else(GuiState::default)
    }

    pub fn store(self, ui: &Ui) {
        ui.memory_mut(|memory| memory.data.insert_temp(Id::NULL, self));
    }
}
