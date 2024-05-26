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
            let mut frame = egui::Frame::none().fill(TRACK_HEADER_COLOR);
            frame.inner_margin = Margin::ZERO;
            frame.outer_margin = Margin::ZERO;
            frame.show(ui, |ui| {
                ui.set_min_width(TRACK_HEADER_WIDTH);

                let label = egui::Label::new(format!("Track {}", index));
                ui.add_sized(vec2(30.0, TRACK_HEIGHT), label);

                let port = data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .track_manager
                    .get_track_port(index)
                    .to_string();

                ui.add_space(20.0);

                let text_input_rect = egui::vec2(10.0, ui.available_height());

                port_selector(ui, text_input_rect, port, &data_layer, index);

                channel_selector(data_layer, index, ui, text_input_rect);
            });

            let clip_frame = egui::Frame::none().fill(egui::Color32::GREEN);
            clip_frame.show(ui, |ui| {
                clip(ctx, ui, &index.to_string());
            });
            // test_step_sequencer(ui, data_layer, index);
        });
    });
}

fn port_selector(
    ui: &mut Ui,
    text_input_rect: Vec2,
    mut port: String,
    data_layer: &Arc<Mutex<DataLayer>>,
    index: usize,
) {
    let port_selector = ui.add_sized(text_input_rect, egui::TextEdit::singleline(&mut port));
    if port_selector.lost_focus() || port_selector.changed() {
        match port.parse::<u8>() {
            Ok(value) => {
                data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .track_manager
                    .tracks
                    .get_mut(index)
                    .unwrap()
                    .set_port(value);
            }
            Err(_) => tracing::warn!("port must be a number"),
        }
    }
}

fn channel_selector(
    data_layer: Arc<Mutex<DataLayer>>,
    index: usize,
    ui: &mut Ui,
    text_input_rect: Vec2,
) {
    let mut channel = data_layer
        .lock()
        .unwrap()
        .project_manager
        .track_manager
        .tracks
        .get(index)
        .unwrap()
        .instrument
        .channel
        .to_string();

    let channel_selector = egui::TextEdit::singleline(&mut channel);
    let response = ui.add_sized(text_input_rect, channel_selector);
    if response.lost_focus() || response.changed() {
        match channel.parse::<u8>() {
            Ok(value) => {
                data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .track_manager
                    .tracks
                    .get_mut(index)
                    .unwrap()
                    .set_channel(value);
            }
            Err(_) => tracing::warn!("port must be a number"),
        }
    }
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
