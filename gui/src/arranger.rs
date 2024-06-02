use crate::ui::{self, common::TRACK_COLOR};
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
    let track = ui::TrackWidget::new(Arc::clone(&data_layer), index).fill(TRACK_COLOR);
    let response = track.show(ui, ctx);
    if response.clicked() {
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
