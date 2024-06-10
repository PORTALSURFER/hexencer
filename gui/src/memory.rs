use egui::{Id, Pos2, Ui};
use hexencer_core::data::ClipId;

/// gui wide shared state, used for storing interaction states used throughout the gui
#[derive(Clone, Default)]
pub struct GuiState {
    /// id of the currently selected clip
    pub selected_clip: Option<ClipId>,
    /// stores the last known clip location when drag_dropped
    pub last_dragged_clip_pos: Option<Pos2>,
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

/// state of the 'ClipWidget'
// TODO merge these state types into a generic type
// #[derive(Clone, Copy, Debug, Default)]

pub trait WidgetState
where
    Self: Clone + Default + Sync + Send + 'static,
{
    /// load this state from memory, or create a default one
    fn load_or_default(id: Id, ui: &Ui) -> Self {
        ui.memory(|mem| mem.data.get_temp(id).unwrap_or_default())
    }

    /// load this state from memory, or create a default one
    fn load(id: Id, ui: &Ui) -> Option<Self> {
        ui.memory(|mem| mem.data.get_temp(id))
    }

    /// store this state to memory
    fn store(self, id: Id, ui: &mut Ui) {
        ui.memory_mut(|mem| mem.data.insert_temp(id, self))
    }
}
