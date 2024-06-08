use crate::ui::{self, common::TRACK_COLOR};
use egui::{layers::ShapeIdx, Color32, Ui};
use hexencer_core::{data::DataInterface, TrackId};

/// creates a new track ui element
pub fn track(data_layer: DataInterface, ctx: &egui::Context, index: TrackId, ui: &mut egui::Ui) {
    let track = ui::TrackWidget::new(data_layer, index).fill(TRACK_COLOR);
    track.show(ui, ctx, || {
        tracing::info!("a clip was dropped on this track")

        // let mut data = self.data_layer.get();
        // TODO move this over to commander
        // {
        //     let clip = data.project_manager.move_clip(clip_id, &self.track_id);
        //     if let Some(clip) = clip {
        //         if let Some(track) = data.project_manager.tracks.get_mut(self.track_id) {
        //             let tick = (pos.x - rect.min.x) / 24.0 * 120.0;
        //             tracing::info!("pos {}", pos.x);
        //             track.add_clip(Tick::from(tick), clip);
        //         }
        //     }
        // }
    });
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
