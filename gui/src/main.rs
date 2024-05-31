#![deny(missing_docs)]
#![allow(dead_code)]

//! the main entry point for the application

mod arranger;
mod memory;
mod ui;
mod viewport;

use egui::Color32;
use hexencer_core::data::DataLayer;
use hexencer_engine::midi::{MidiEngine, MidiEngineSender};
use hexencer_engine::{Sequencer, SequencerCommand, SequencerSender};
use std::sync::{Arc, Mutex};
use tokio::task;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use viewport::MainViewport;

pub use hexencer_core::DataId;

/// color used for all regular edges in the ui
pub const EDGE_COLOR: Color32 = Color32::from_rgb(20, 20, 20);

fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("hexencer started");
}

fn start_midi_engine() -> MidiEngineSender {
    let (midi_sender, midi_receiver) = tokio::sync::mpsc::unbounded_channel();
    let midi_engine = MidiEngine::new();
    task::spawn(midi_engine.listen(midi_receiver));
    midi_sender
}

fn start_sequencer_engine(
    midi_sender: MidiEngineSender,
    data_layer: Arc<Mutex<DataLayer>>,
) -> SequencerSender {
    let (sequencer_sender, sequencer_receiver) = tokio::sync::mpsc::unbounded_channel();
    let sequencer = Sequencer::new(Arc::clone(&data_layer), midi_sender);
    task::spawn(sequencer.listen(sequencer_receiver));
    sequencer_sender
}

#[tokio::main]
async fn main() {
    init_logger();

    let data_layer = Arc::new(Mutex::new(DataLayer::default()));
    let midi_engine_sender = start_midi_engine();
    let sequencer_sender = start_sequencer_engine(midi_engine_sender, Arc::clone(&data_layer));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::vec2(1920.0, 1080.0)),

        ..Default::default()
    };

    eframe::run_native(
        "Hexencer",
        options,
        Box::new(|_cc| Box::new(MainViewport::new(data_layer, sequencer_sender))),
    )
    .expect("failed to start eframe app");
}
