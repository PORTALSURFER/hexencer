#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

//! the main entry point for the application

/// arranger part of the gui
mod arranger;
/// gui code
mod gui;
/// utilities for loading and storing data to egui memory
mod memory;
/// ui elements
mod ui;
/// utility code
mod utility;

pub use gui::{options, run};
use hexencer_core::data::DataInterface;
use hexencer_engine::{midi_engine::start_midi_engine, start_sequencer_engine};
pub use memory::WidgetState;
use utility::init_logger;

pub use hexencer_core::DataId;

#[tokio::main]
async fn main() {
    init_logger();

    let data_layer = DataInterface::new();
    let midi_engine_sender = start_midi_engine();
    let sequencer_sender = start_sequencer_engine(midi_engine_sender, data_layer.clone());

    let options = options();
    run(options, data_layer, sequencer_sender);
}
