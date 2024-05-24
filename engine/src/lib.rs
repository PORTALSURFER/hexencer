use hexencer_core::{data::DataLayer, Instrument, MidiEvent, MidiMessage, Note};
use midi::MidiEngineSender;
use std::{sync::Arc, sync::Mutex, time::Duration};
use tokio::{task, time};

pub mod midi;

pub enum SequencerCommand {
    Play,
    Stop,
}

#[derive(Default)]
pub struct Sequencer {
    data_layer: Arc<Mutex<DataLayer>>,
    midi_engine_sender: Option<MidiEngineSender>,
    bpm: f64,
    ppqn: u32,
    running: Arc<Mutex<bool>>,
    current_tick: u64,
}

impl Sequencer {
    pub fn new(data_layer: Arc<Mutex<DataLayer>>, midi_engine_sender: MidiEngineSender) -> Self {
        Self {
            data_layer,
            midi_engine_sender: Some(midi_engine_sender),
            bpm: 120.0,
            ppqn: 480,
            current_tick: 0,
            running: Arc::new(Mutex::new(false)),
        }
    }

    fn tick_duration(&self) -> u64 {
        let beat_duration = 60.0 / self.bpm;
        let tick_duration = (beat_duration / self.ppqn as f64) * 1000.0;
        tick_duration as u64
    }

    pub async fn process(
        mut self,
        mut command_receiver: tokio::sync::mpsc::UnboundedReceiver<SequencerCommand>,
    ) {
        println!("sequencer listening for commands");
        let mut interval = time::interval(Duration::from_millis(self.tick_duration()));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if *self.running.lock().unwrap() {
                        // println!("tick .. {}", self.current_tick);
                        let events = self.data_layer.lock().unwrap().project_manager.get_all_events();
                        let current_events: Vec<MidiEvent> = events.into_iter().filter(|event| event.tick == self.current_tick && event.on).collect();

                        self.play_events(self.current_tick, current_events);
                        self.current_tick = self.current_tick + 1;
                    }
                }
                Some(command) = command_receiver.recv() => {
                    match command {
                        SequencerCommand::Play => {
                            println!("play commnd received");
                            *self.running.lock().unwrap() = true;
                        }
                        SequencerCommand::Stop => {
                            println!("stop");
                            *self.running.lock().unwrap() = false;
                        }
                    }

                }
            }
        }
    }

    #[deprecated]
    pub async fn play(
        data_layer: Arc<Mutex<DataLayer>>,
        running: Arc<Mutex<bool>>,
        tick_duration: u64,
    ) {
        println!("playing sequencer");
        // let data_layer = data_layer.lock().unwrap();
        // let mut midi_events = data_layer.get_midi_events();
    }

    fn play_events(&mut self, current_tick: u64, events: Vec<MidiEvent>) {
        for event in events {
            if let Some(sender) = &mut self.midi_engine_sender {
                sender.send(event.midi_message).unwrap();
            }
        }
    }
}
