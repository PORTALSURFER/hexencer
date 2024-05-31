use crate::{
    memory::GuiState,
    ui::{self, common::TRACK_COLOR},
};
use egui::{layers::ShapeIdx, Color32, Ui};
use hexencer_core::{data::DataLayer, Tick};
use std::sync::{Arc, Mutex};

/// creates a new track ui element
pub fn track(
    data_layer: Arc<Mutex<DataLayer>>,
    ctx: &egui::Context,
    index: usize,
    ui: &mut egui::Ui,
) {
    let track = ui::TrackWidget::new().fill(TRACK_COLOR);
    let response = track.show(ui, |ui| {
        ui.horizontal(|ui| {
            let data = data_layer.lock().unwrap();
            let track = data.project_manager.tracks.get(index);
            if let Some(track) = track {
                for (tick, clip) in &track.clips {
                    if ui::clip(ctx, ui, clip.get_id(), *tick).drag_started() {
                        tracing::info!("clicked {}", clip.get_id().to_string());

                        let mut gui_state = GuiState::load(ui);
                        gui_state.selected_clip = Some(clip.get_id());
                        gui_state.store(ui);
                    };
                }
            }
        });
    });

    if response.response.clicked() {
        tracing::info!("Track clicked!");
        data_layer
            .lock()
            .unwrap()
            .add_clip(index, Tick::from(6 * 480), "new clip");
    }
}

/// gui representation of track
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct TrackWidget {
    /// draw height out the track
    height: f32,
    /// color used to paint the track
    fill: Color32,
}

impl TrackWidget {
    /// creates a new track widget
    pub fn new() -> Self {
        Self::default()
    }

    /// set the fill color   
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }
}

/// intermediate type housing the prepared trackwidget before painting
struct Prepared {
    /// reference to the track widget this object represents
    track: TrackWidget,
    /// placeholder for the background shape
    where_to_put_background: ShapeIdx,
    /// inner ui
    content_ui: Ui,
}
