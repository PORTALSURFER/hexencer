#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

//! the main entry point for the application

/// arranger part of the gui
mod arranger;
/// utilities for loading and storing data to egui memory
mod memory;
/// ui elements
mod ui;
/// the main viewport
mod viewport;

use egui::{Color32, IconData};
use hexencer_core::data::DataLayer;
use hexencer_engine::midi_engine::{MidiEngine, MidiEngineSender};
use hexencer_engine::{Sequencer, SequencerSender};
use std::sync::{Arc, Mutex};
use tokio::task;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use viewport::MainViewport;

pub use hexencer_core::DataId;

/// color used for all regular edges in the ui
pub const EDGE_COLOR: Color32 = Color32::from_rgb(20, 20, 20);

/// initialize the logging system
fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("hexencer started");
}

/// starts up the midi engine and listens for commands, return the sender to send commands to the midi engine
fn start_midi_engine() -> MidiEngineSender {
    let (midi_sender, midi_receiver) = tokio::sync::mpsc::unbounded_channel();
    let midi_engine = MidiEngine::new();
    task::spawn(midi_engine.listen(midi_receiver));
    midi_sender
}

/// starts up the sequencer engine and listens for commands, returns the sender to send commands to the sequencer
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

    // let icon = load_icon("assets/logo.png");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // .with_icon(icon)
            .with_inner_size(egui::vec2(800.0, 600.0)),

        ..Default::default()
    };

    eframe::run_native(
        "Hexencer",
        options,
        Box::new(|_cc| Box::new(MainViewport::new(data_layer, sequencer_sender))),
    )
    .expect("failed to start eframe app");
}

/// load icon from png image
fn load_icon(path: &str) -> IconData {
    let image = image::open(path).expect("failed to open icon image");
    if let Some(image) = image.as_rgba8() {
        let (width, height) = image.dimensions();
        let rgba = image.clone().into_vec();
        IconData {
            rgba,
            width,
            height,
        }
    } else {
        IconData::default()
    }
}
