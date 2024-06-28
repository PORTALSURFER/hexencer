use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use hexencer_core::{data::StorageInterface, Tick};
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
        Self {
            running: false,
            current_tick: Tick::zero(),
            ppqn: 480,
        }
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

    // /// starts listening for and processing commands
    // pub async fn listen(mut self, mut command_receiver: SequencerReceiver) {
    //     tracing::info!("sequencer listening for commands");
    //     let mut interval = time::interval(Duration::from_micros(self.tick_duration()));
    //     loop {
    //         tokio::select! {
    //             _ = interval.tick() => {
    //                     let mut state = self.state.write().unwrap();
    //                     // self.process_events();
    //                     state.current_tick.tick();
    //                     if let Ok(mut storage) = self.storage.write() { storage.set_tick(state.current_tick) }
    //             },
    //             Some(command) = command_receiver.recv() => {
    //                 match command {
    //                     SequencerCommand::Play => {
    //                         tracing::info!("play command received");
    //                         self.play().await;
    //                     }
    //                     SequencerCommand::Stop => {
    //                         tracing::info!("stop");
    //                         self.stop().await;
    //                     }
    //                     SequencerCommand::Reset => {
    //                         let mut state = self.state.write().unwrap();
    //                         tracing::info!("reset");
    //                         state.current_tick.reset();
    //                     }
    //                     SequencerCommand::Pause => {
    //                         let mut state = self.state.write().unwrap();
    //                         tracing::info!("pause");
    //                         state.running = false;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

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
        // let tracks = &self.data_layer.lock().unwrap().project_manager.tracks;
        // for track in tracks.iter() {
        //     if let Some(event_entry) = track.event_ooolist.get(&self.current_tick) {
        //         for event in event_entry.iter() {
        //             let event_type = event.event_type;
        //             tracing::info!("{} - {}", track, event_type);

        //             if event.is_active {
        //                 let message = event_type.get_message();
        //                 let instrument = &track.instrument;
        //                 self.midi_engine_sender.as_mut().map(|sender| {
        //                     sender.send((message, instrument.port, instrument.channel))
        //                 });
        //             }
        //         }
        //     }
        // }
    }

    async fn reset(&self) {
        let mut state = self.state.write().unwrap();
        state.current_tick = 0.into();
        state.running = false;
    }

    async fn pause(&self) {
        let mut state = self.state.write().unwrap();
        state.running = false;
    }
}

///// starts up the sequencer engine and listens for commands, returns the sender to send commands to the sequencer
// pub fn start_sequencer_engine(
//     midi_sender: MidiEngineSender,
//     data_layer: StorageInterface,
// ) -> SequencerSender {
//     let (sequencer_sender, sequencer_receiver) = tokio::sync::mpsc::unbounded_channel();
//     let sequencer = Sequencer::new(data_layer, midi_sender);
//     task::spawn(sequencer.listen(sequencer_receiver));
//     sequencer_sender
// }
