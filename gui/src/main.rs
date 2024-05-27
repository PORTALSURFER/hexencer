#![deny(missing_docs)]
#![allow(dead_code)]

//! the main entry point for the application

mod arranger;
mod ui;

use arranger::{track, SELECTED_CLIP};
use egui::{vec2, Frame, Margin, Ui, Vec2};
use hexencer_core::data::DataLayer;
use hexencer_engine::midi::MidiEngine;
use hexencer_engine::{Sequencer, SequencerCommand, SequencerSender};
use std::sync::{Arc, Mutex};
use tokio::task;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ui::common::{TRACK_HEADER_COLOR, TRACK_HEIGHT};
use ui::Timeline;

pub use hexencer_core::DataId;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("hexencer started");

    let data_layer = Arc::new(Mutex::new(DataLayer::default()));

    let (sequencer_sender, sequencer_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (midi_sender, midi_receiver) = tokio::sync::mpsc::unbounded_channel();

    let midi_engine = MidiEngine::new();
    task::spawn(midi_engine.listen(midi_receiver));

    let sequencer = Sequencer::new(Arc::clone(&data_layer), midi_sender);
    task::spawn(sequencer.listen(sequencer_receiver));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1920.0, 1080.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Hexencer",
        options,
        Box::new(|_cc| Box::new(Gui::new(data_layer, sequencer_sender))),
    )
    .expect("failed to start eframe app");
}

#[derive(Default)]
struct Gui {
    data_layer: Arc<Mutex<DataLayer>>,
    sequencer_sender: Option<SequencerSender>,
}

impl Gui {
    fn new(
        // cc: &eframe::CreationContext<'_>,
        data_layer: Arc<Mutex<DataLayer>>,
        sender: SequencerSender,
    ) -> Self {
        Self {
            data_layer,
            sequencer_sender: Some(sender),
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|style| {
            style.spacing.item_spacing = vec2(0.0, 0.0);
            style.spacing.window_margin = Margin::ZERO;
            style.spacing.indent = 0.0
        });

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.label("toolbar menu"));
        });
        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.label("status info"));
        });

        egui::SidePanel::left("tracks")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    let track_ids: Vec<usize> = self
                        .data_layer
                        .lock()
                        .unwrap()
                        .project_manager
                        .tracks
                        .iter()
                        .map(|track| track.id)
                        .collect();

                    for id in track_ids {
                        let mut frame = egui::Frame::none().fill(TRACK_HEADER_COLOR);
                        frame.inner_margin = Margin::ZERO;
                        frame.outer_margin = Margin::ZERO;
                        frame.show(ui, |ui| {
                            // ui.set_min_width(TRACK_HEADER_WIDTH);

                            let label = egui::Label::new(format!("Track {}", id));
                            ui.add_sized(vec2(24.0, TRACK_HEIGHT), label);

                            // let port = self
                            //     .data_layer
                            //     .lock()
                            //     .unwrap()
                            //     .project_manager
                            //     .tracks
                            //     .get_track_port(id)
                            //     .to_string();
                            // // ui.add_space(16.0);
                            // let text_input_rect = egui::vec2(8.0, 24.0);
                            // let clone = Arc::clone(&self.data_layer);
                            // port_selector(ui, text_input_rect, port, &self.data_layer, id);
                            // channel_selector(clone, id, ui, text_input_rect);
                        });
                    }
                });
            });

        egui::SidePanel::right("info")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("info");
                if ui.button("add track").clicked() {
                    self.data_layer.lock().unwrap().project_manager.add_track();
                }
                if ui.button("remove track").clicked() {
                    self.data_layer
                        .lock()
                        .unwrap()
                        .project_manager
                        .remove_track();
                }
                if ui.button("play").clicked() {
                    self.sequencer_sender.as_mut().map(|sender| {
                        let _ = sender.send(SequencerCommand::Play);
                    });
                }
                if ui.button("stop").clicked() {
                    self.sequencer_sender.as_mut().map(|sender| {
                        let _ = sender.send(SequencerCommand::Stop);
                    });
                }
                if ui.button("reset").clicked() {
                    self.sequencer_sender.as_mut().map(|sender| {
                        let _ = sender.send(SequencerCommand::Reset);
                    });
                }
            });

        let editor_height = 200.0;
        egui::TopBottomPanel::bottom("editor")
            .min_height(editor_height)
            .default_height(editor_height)
            .resizable(true)
            .show(ctx, |ui| {
                if let Some(selected_clip_id) =
                    ui.memory(|mem| mem.data.get_temp::<DataId>(SELECTED_CLIP.into()))
                {
                    if let Some(selected_clip) = self
                        .data_layer
                        .lock()
                        .unwrap()
                        .project_manager
                        .find_clip(selected_clip_id)
                    {
                        ui.label(format!("Selected clip: {}", selected_clip.name));
                    }
                }

                ui.centered_and_justified(|ui| ui.label("editor"));
            });

        let mut frame = Frame::none();
        frame.outer_margin = Margin::ZERO;
        frame.inner_margin = Margin::ZERO;
        let fill = egui::Color32::from_rgb(20, 20, 20);
        frame = frame.fill(fill);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            Timeline::new(10.0).show(ui);
            ui.vertical(|ui| {
                let track_ids: Vec<usize> = self
                    .data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .tracks
                    .iter()
                    .map(|track| track.id)
                    .collect();

                for id in track_ids {
                    let clone = Arc::clone(&self.data_layer);
                    track(clone, ctx, id, ui);
                }
            });
        });
    }
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
                    .tracks
                    .get_mut(index)
                    .unwrap()
                    .set_channel(value);
            }
            Err(_) => tracing::warn!("port must be a number"),
        }
    }
}
