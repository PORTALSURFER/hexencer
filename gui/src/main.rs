pub mod track;
pub mod ui;

use hexencer_core::data::DataLayer;
use hexencer_engine::midi::MidiEngine;
use hexencer_engine::{Sequencer, SequencerCommand};
use std::sync::{Arc, Mutex};
use tokio::task;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use track::track_ui;

type SequencerSender = tokio::sync::mpsc::UnboundedSender<SequencerCommand>;
type SequencerReceiver = tokio::sync::mpsc::UnboundedReceiver<SequencerCommand>;

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
                    track_ui(clone, ctx, id, ui);
                }
            });
        });
    }
}
