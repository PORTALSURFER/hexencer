use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use hexencer_core::{
    data::{MidiMessage, StorageInterface},
    Tick,
};
use tokio::{task, time};

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
}

/// the 'Sequencer' keep track of the tick and processes events ensuring they are sent to the right engine
#[derive(Default, Debug)]
pub struct Sequencer {
    /// the data layer, used to store and retreive projects, etc
    data: StorageInterface,
    /// use this to send commands to the midi engine, like playing a note
    midi_engine_sender: Option<MidiEngineSender>,
    /// current bpm of the sequencer
    bpm: f64,
    /// parts per quarter note, how many ticks per beat
    ppqn: u32,
    /// true if the sequencer is running
    running: Arc<Mutex<bool>>,
    /// current tick, position of the playhead
    current_tick: Tick,
}

impl Sequencer {
    /// creates a new 'Sequencer'
    pub fn new(data_layer: StorageInterface, midi_engine_sender: MidiEngineSender) -> Self {
        let bpm = data_layer.read().unwrap().bpm();
        Self {
            data: data_layer,
            midi_engine_sender: Some(midi_engine_sender),
            bpm,
            ppqn: 480,
            current_tick: Tick::zero(),
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// calculate the duration of a tick
    fn tick_duration(&self) -> u64 {
        let beat_duration = 60.0 / self.bpm;
        let tick_duration = (beat_duration / self.ppqn as f64) * 1000.0;
        (tick_duration * 1000.0) as u64
    }

    /// starts listening for and processing commands
    pub async fn listen(mut self, mut command_receiver: SequencerReceiver) {
        tracing::info!("sequencer listening for commands");
        let mut interval = time::interval(Duration::from_micros(self.tick_duration()));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if *self.running.lock().unwrap() {
                        self.process_events();
                        self.current_tick.tick();
                        if let Ok(mut data) = self.data.write() { data.set_tick(self.current_tick) }
                    }
                }
                Some(command) = command_receiver.recv() => {
                    match command {
                        SequencerCommand::Play => {
                            tracing::info!("play command received");
                            self.play();
                        }
                        SequencerCommand::Stop => {
                            tracing::info!("stop");
                            self.stop();
                        }
                        SequencerCommand::Reset => {
                            tracing::info!("reset");
                            self.current_tick.reset();
                            self.stop();
                        }
                    }
                }
            }
        }
    }

    /// sends stop signals to both midi ports
    fn stop(&mut self) {
        *self.running.lock().unwrap() = false;

        if let Some(sender) = self.midi_engine_sender.as_mut() {
            sender.send((MidiMessage::AllNoteOff, 0, 0)).unwrap()
        }
        if let Some(sender) = self.midi_engine_sender.as_mut() {
            sender.send((MidiMessage::AllNoteOff, 0, 0)).unwrap()
        }
    }

    /// start playing the sequencer
    fn play(&mut self) {
        *self.running.lock().unwrap() = true;
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
}

/// starts up the sequencer engine and listens for commands, returns the sender to send commands to the sequencer
pub fn start_sequencer_engine(
    midi_sender: MidiEngineSender,
    data_layer: StorageInterface,
) -> SequencerSender {
    let (sequencer_sender, sequencer_receiver) = tokio::sync::mpsc::unbounded_channel();
    let sequencer = Sequencer::new(data_layer, midi_sender);
    task::spawn(sequencer.listen(sequencer_receiver));
    sequencer_sender
}
