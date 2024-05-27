use std::sync::{Arc, Mutex};

use egui::{layers::ShapeIdx, vec2, Align, Color32, Margin, Ui, Vec2};
use hexencer_core::data::{midi_message::MidiMessage, DataLayer};

use crate::ui::{
    self, clip,
    common::{TRACK_COLOR, TRACK_HEADER_COLOR, TRACK_HEADER_WIDTH, TRACK_HEIGHT},
};

pub fn track_ui(
    data_layer: Arc<Mutex<DataLayer>>,
    ctx: &egui::Context,
    index: usize,
    ui: &mut egui::Ui,
) {
    let track = ui::Track::new().fill(TRACK_COLOR);

    track.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_size(egui::vec2(ui.available_width(), TRACK_HEIGHT));

            let clip_frame = egui::Frame::none().fill(egui::Color32::GREEN);
            clip_frame.show(ui, |ui| {
                if clip(ctx, ui, &index.to_string(), |ui| {
                    ui.label("test");
                })
                .response
                .clicked()
                {
                    tracing::info!("clicked clip");
                };
            });
        });
    });
}

pub fn test_step_sequencer(ui: &mut Ui, data_layer: Arc<Mutex<DataLayer>>, index: usize) {
    ui.horizontal(|ui| {
        let mut changed_triggers = Vec::new();
        let mut data_layer = data_layer.lock().unwrap();
        let track = data_layer
            .project_manager
            .track_manager
            .tracks
            .get_mut(index)
            .unwrap();

        for (tick, event_entry) in &mut track.event_list.iter_mut().filter(|(tick, event_entry)| {
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct Track {
    height: f32,
    fill: Color32,
}

impl Track {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Track {
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }
}

pub struct Prepared {
    pub track: Track,
    where_to_put_background: ShapeIdx,
    pub content_ui: Ui,
}
