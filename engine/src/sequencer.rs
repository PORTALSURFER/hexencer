use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use hexencer_core::{
    data::{ClipKey, StorageInterface},
    Tick,
};
use tokio::time;
use tracing::info;

use crate::midi_engine::MidiEngineSender;

/// used to send a command to a 'Sequencer'
pub type SequencerSender = tokio::sync::mpsc::UnboundedSender<SequencerCommand>;
/// used to receive a command by a 'Sequencer'
pub type SequencerReceiver = tokio::sync::mpsc::UnboundedReceiver<SequencerCommand>;

/// possible 'Sequencer' commands
pub enum SequencerCommand {
    /// start playing the sequencer
    Play,
    /// stop the sequencer
    Stop,
    /// reset the sequencer
    Reset,
    /// pause the sequencer
    Pause,
}

/// hold this to interact with the sequencer
#[derive(Debug)]
pub struct SequencerHandle {
    /// the current state of the sequencer
    pub state: Arc<RwLock<SequencerState>>,
    /// used to send commands to the sequencer
    pub command_sender: SequencerSender,
}

/// the 'Sequencer' keep track of the tick and processes events ensuring they are sent to the right engine
#[derive(Debug)]
pub struct Sequencer {
    /// state of the sequencer, housing current tick etc
    pub state: Arc<RwLock<SequencerState>>,
    /// the data layer, used to store and retreive projects, etc
    storage: StorageInterface,
    /// use this to send commands to the midi engine, like playing a note
    midi_engine_sender: MidiEngineSender,
    /// this is used to receive any commands for the sequencer to process
    command_receiver: SequencerReceiver,
}

#[derive(Debug)]
pub struct SequencerState {
    /// true if the sequencer is running
    running: bool,
    /// current tick, position of the playhead
    pub current_tick: Tick,
    /// parts per quarter note, how many ticks per beat
    ppqn: u32,
}

impl SequencerState {
    /// creates new sequencer state
    pub fn new() -> Self {
        Self { running: false, current_tick: Tick::zero(), ppqn: 480 }
    }
}

impl Default for SequencerState {
    fn default() -> Self {
        Self::new()
    }
}

impl Sequencer {
    /// creates a new 'Sequencer'
    pub fn new(
        storage: StorageInterface,
        midi_engine_sender: MidiEngineSender,
        command_receiver: SequencerReceiver,
    ) -> Self {
        Self {
            state: Arc::new(RwLock::new(SequencerState::new())),
            storage,
            midi_engine_sender,
            command_receiver,
        }
    }

    /// run the sequencer
    pub async fn run(mut self) {
        let mut interval = time::interval(Duration::from_micros(self.tick_duration()));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.tick().await;
                }
                Some(command) = self.command_receiver.recv() => {
                    self.handle_command(command).await;
                }
            }
        }
    }

    /// handles ticking of the sequencer
    async fn tick(&mut self) {
        if self.state.read().unwrap().running {
            self.process_events();
            let mut state = self.state.write().unwrap();
            state.current_tick.tick();
        }
    }

    /// handles processing of commands
    async fn handle_command(&mut self, command: SequencerCommand) {
        match command {
            SequencerCommand::Play => {
                self.play().await;
            }
            SequencerCommand::Stop => {
                self.stop().await;
            }
            SequencerCommand::Reset => {
                self.reset().await;
            }
            SequencerCommand::Pause => {
                self.pause().await;
            }
        }
    }

    /// calculate the duration of a tick
    fn tick_duration(&self) -> u64 {
        let state = self.state.read().unwrap();
        let storage = self.storage.read().unwrap();
        let bpm = storage.bpm();
        let beat_duration = 60.0 / bpm;
        let tick_duration = (beat_duration / state.ppqn as f64) * 1000.0;
        (tick_duration * 1000.0) as u64
    }

    /// sends stop signals to both midi ports
    async fn stop(&mut self) {
        let mut state = self.state.write().unwrap();
        state.running = false;
    }

    /// start playing the sequencer
    async fn play(&mut self) {
        let mut state = self.state.write().unwrap();
        state.running = true;
    }

    /// process events at the current tick, sending them to the midi engine
    fn process_events(&mut self) {
        let storage = self.storage.read().unwrap();
        let state = self.state.read().unwrap();
        let tracks = &storage.project_manager.track_collection;

        for track in tracks.iter() {
            for (key, clip) in track.clip_collection.iter() {
                for (tick, event_segments) in clip.events.iter() {
                    if *tick == state.current_tick {
                        for segment in event_segments {
                            info!("tick:{} process event {:?}", tick, segment);
                        }
                    }
                }
            }
        }
    }

    /// reset the sequencer
    async fn reset(&self) {
        let mut state = self.state.write().unwrap();
        state.current_tick = 0.into();
        state.running = false;
    }

    /// pause the sequencer
    async fn pause(&self) {
        let mut state = self.state.write().unwrap();
        state.running = false;
    }
}
