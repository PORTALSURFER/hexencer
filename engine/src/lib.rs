use hexencer_core::{data::DataLayer, Instrument, MidiEvent, MidiMessage, Note, Track};
use midi::MidiEngineSender;
use std::{sync::Arc, sync::Mutex, time::Duration};
use tokio::{task, time};

pub mod midi;

pub enum SequencerCommand {
    Play,
    Stop,
    Reset,
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
    ) -> ! {
        println!("sequencer listening for commands");
        let mut interval = time::interval(Duration::from_millis(self.tick_duration()));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if *self.running.lock().unwrap() {
                        // println!("tick .. {}", self.current_tick);
                        let events = self.data_layer.lock().unwrap().project_manager.get_all_events();
                        let current_events: Vec<MidiEvent> = events.into_iter().filter(|event| event.tick == self.current_tick && event.on).collect();
                        self.send_to_midi_engine(self.current_tick, current_events);
                        self.current_tick = self.current_tick + 1;
                    }
                }
                Some(command) = command_receiver.recv() => {
                    match command {
                        SequencerCommand::Play => {
                            println!("play commnd received");
                            self.play();
                        }
                        SequencerCommand::Stop => {
                            println!("stop");
                            self.stop();
                        }
                        SequencerCommand::Reset => {
                            println!("reset");
                            self.current_tick = 0;
                            self.stop();
                        }
                    }
                }
            }
        }
    }

    fn stop(&mut self) {
        *self.running.lock().unwrap() = false;

        let instrument = Instrument::new("piano", 0);
        self.midi_engine_sender
            .as_mut()
            .map(|sender| sender.send(MidiEvent::global_note_off(instrument)).unwrap());
        let instrument = Instrument::new("piano", 1);
        self.midi_engine_sender
            .as_mut()
            .map(|sender| sender.send(MidiEvent::global_note_off(instrument)).unwrap());
    }

    fn play(&mut self) {
        *self.running.lock().unwrap() = true;
    }

    fn send_to_midi_engine(&mut self, current_tick: u64, events: Vec<MidiEvent>) {
        for event in events {
            if let Some(sender) = &mut self.midi_engine_sender {
                let current_beat = current_tick / self.ppqn as u64;
                println!("[{}] - {}", current_beat, event);
                sender.send(event).unwrap();
            }
        }
    }
}
