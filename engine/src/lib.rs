#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

//! houses the midi engine

/// midi engine
pub mod midi_engine;
/// sequencer engine
mod sequencer;

// pub use sequencer::start_sequencer_engine;
pub use sequencer::Sequencer;
pub use sequencer::SequencerCommand;
pub use sequencer::SequencerHandle;
pub use sequencer::SequencerSender;
