use std::sync::{Arc, Mutex};

use hexencer_core::data::{midi_message::MidiMessage, DataLayer};

const TRACK_COLOR: egui::Color32 = egui::Color32::from_rgb(32, 42, 42);
const TRACK_HEADER_COLOR: egui::Color32 = egui::Color32::from_rgb(54, 54, 74);
const TRACK_HEIGHT: f32 = 25.0;
const TRACK_HEADER_WIDTH: f32 = 100.0;

pub fn track_ui(
    data_layer: Arc<Mutex<DataLayer>>,
    ctx: &egui::Context,
    index: usize,
    ui: &mut egui::Ui,
) {
    egui::Frame::none().fill(TRACK_COLOR).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_size(egui::vec2(ui.available_width(), TRACK_HEIGHT));
            egui::Frame::none().fill(TRACK_HEADER_COLOR).show(ui, |ui| {
                ui.set_min_width(TRACK_HEADER_WIDTH);
                ui.label(format!("Track {}", index));
                let mut port = data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .track_manager
                    .tracks
                    .get(index)
                    .unwrap()
                    .instrument
                    .port
                    .to_string();

                let text_input_rect = egui::vec2(20.0, ui.available_height() - 4.0);
                let port_selector =
                    ui.add_sized(text_input_rect, egui::TextEdit::singleline(&mut port));

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
                        Err(_) => println!("port can only be a number"),
                    }
                }
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
                        Err(_) => println!("channel can only be a number"),
                    }
                }
            });
            ui.horizontal(|ui| {
                let mut changed_triggers = Vec::new();
                let mut data_layer = data_layer.lock().unwrap();
                let track = data_layer
                    .project_manager
                    .track_manager
                    .tracks
                    .get_mut(index)
                    .unwrap();

                for (tick, event_entry) in
                    &mut track.event_list.iter_mut().filter(|(tick, event_entry)| {
                        let hexencer_core::event::Event::Midi(message) = &event_entry.event;
                        match message {
                            MidiMessage::NoteOn { key, velocity } => true,
                            _ => false,
                        }
                    })
                {
                    if ui
                        .checkbox(&mut event_entry.active, format!("{}", tick.as_beat()))
                        .changed()
                    {
                        changed_triggers.push(tick.clone());
                    }
                }
            });
        });
    });
}

fn clip(ctx: &egui::Context, ui: &mut egui::Ui) {
    let id = egui::Id::from("new clip");
    // ui.button("Clip");
    egui::Area::new(id)
        .movable(true)
        .constrain_to(ui.max_rect())
        .show(ctx, |ui| {
            egui::Frame::none().fill(egui::Color32::RED).show(ui, |ui| {
                ui.allocate_space(egui::vec2(10.0, ui.available_height() - 15.0));
                //ui.add(egui::Label::new("Clip").selectable(false));
            });
        });
}
