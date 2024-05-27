use std::sync::{Arc, Mutex};

use egui::{layers::ShapeIdx, vec2, Align, Color32, Margin, Ui, Vec2};
use hexencer_core::data::{DataLayer, MidiMessage};

use crate::ui::{
    self, clip,
    common::{TRACK_COLOR, TRACK_HEADER_COLOR, TRACK_HEADER_WIDTH, TRACK_HEIGHT},
};

/// creates a new track ui element
pub fn track(
    data_layer: Arc<Mutex<DataLayer>>,
    ctx: &egui::Context,
    index: usize,
    ui: &mut egui::Ui,
) {
    let track = ui::Track::new().fill(TRACK_COLOR);
    track.show(ctx, ui, |ui| {
        ui.horizontal(|ui| {
            let data = data_layer.lock().unwrap();
            let track = data.project_manager.tracks.get(index);
            if let Some(track) = track {
                for (tick, clip) in &track.clips {
                    ui::clip(ctx, ui, &clip.name, *tick);
                }
            }
            // ui.set_min_size(egui::vec2(ui.available_width(), TRACK_HEIGHT));

            // let clip_frame = egui::Frame::none().fill(egui::Color32::GREEN);
            // clip_frame.show(ui, |ui| {
            // if clip(ctx, ui, &index.to_string()).clicked() {
            // tracing::info!("clicked clip");
            // };
            // });
        });
    });
}

fn _test_step_sequencer(ui: &mut Ui, data_layer: Arc<Mutex<DataLayer>>, index: usize) {
    ui.horizontal(|ui| {
        let mut changed_triggers = Vec::new();
        let mut data_layer = data_layer.lock().unwrap();
        let track = data_layer.project_manager.tracks.get_mut(index).unwrap();

        for (tick, event_entry) in &mut track.event_list.iter_mut().filter(|(_, event_entry)| {
            let hexencer_core::event::Event::Midi(message) = &event_entry.event;
            match message {
                MidiMessage::NoteOn { key, velocity } => true,
                _ => false,
            }
        }) {
            if ui
                .checkbox(&mut event_entry.active, format!("{}", tick.as_beat()))
                .changed()
            {
                changed_triggers.push(tick.clone());
            }
        }
    });
}

/// gui representation of track
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct Track {
    height: f32,
    fill: Color32,
}

impl Track {
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

struct Prepared {
    track: Track,
    where_to_put_background: ShapeIdx,
    content_ui: Ui,
}
