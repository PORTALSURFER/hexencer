use std::sync::{Arc, Mutex};

use hexencer_core::data::midi_message::MidiMessage;
use hexencer_core::data::trig_list::EventList;
use hexencer_core::data::DataLayer;
use hexencer_core::trig::Event::Midi;
use hexencer_engine::midi::MidiEngine;
use hexencer_engine::{Sequencer, SequencerCommand};

use tokio::task;

type SequencerSender = tokio::sync::mpsc::UnboundedSender<SequencerCommand>;
type SequencerReceiver = tokio::sync::mpsc::UnboundedReceiver<SequencerCommand>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let data_layer = Arc::new(Mutex::new(DataLayer::default()));

    let (sequencer_sender, sequencer_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (midi_sender, midi_receiver) = tokio::sync::mpsc::unbounded_channel();

    let midi_engine = MidiEngine::new();
    task::spawn(midi_engine.process(midi_receiver));

    let sequencer = Sequencer::new(Arc::clone(&data_layer), midi_sender);
    task::spawn(sequencer.process(sequencer_receiver));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hexencer",
        options,
        Box::new(|cc| Box::new(Gui::new(cc, data_layer, sequencer_sender))),
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
        cc: &eframe::CreationContext<'_>,
        data_layer: Arc<Mutex<DataLayer>>,
        sender: SequencerSender,
    ) -> Self {
        Self {
            data_layer,
            sequencer_sender: Some(sender),
        }
    }
}

const TRACK_HEIGHT: f32 = 25.0;
const TRACK_HEADER_WIDTH: f32 = 100.0;

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.label("some toolbar");
        });
        egui::SidePanel::left("info").show(ctx, |ui| {
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let track_ids: Vec<usize> = self
                    .data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .track_manager
                    .tracks
                    .iter()
                    .map(|track| track.id)
                    .collect();

                for id in track_ids {
                    let clone = Arc::clone(&self.data_layer);
                    new_track(clone, ctx, id, ui);
                }
            });
        });
    }
}

const TRACK_COLOR: egui::Color32 = egui::Color32::from_rgb(32, 42, 42);
const TRACK_HEADER_COLOR: egui::Color32 = egui::Color32::from_rgb(54, 54, 74);

fn new_track(
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

                let port_selector = ui.text_edit_singleline(&mut port);

                if port_selector.lost_focus() || port_selector.changed() {
                    match port.parse::<u8>() {
                        Ok(value) => {
                            println!("yay");
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
                        let Midi(message) = &event_entry.event;
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

                // if !changed_triggers.is_empty() {
                //     let track = &mut data_layer.project_manager.track_manager.tracks[index];
                //     track.event_list = track.trigs.build_event_list();
                // }
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
