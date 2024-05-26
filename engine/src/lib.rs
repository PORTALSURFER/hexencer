use hexencer_core::{
    data::{midi_message::MidiMessage, DataLayer},
    Tick,
};
use midi::MidiEngineSender;
use std::{sync::Arc, sync::Mutex, time::Duration};
use tokio::time;

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
    current_tick: Tick,
}

impl Sequencer {
    pub fn new(data_layer: Arc<Mutex<DataLayer>>, midi_engine_sender: MidiEngineSender) -> Self {
        Self {
            data_layer,
            midi_engine_sender: Some(midi_engine_sender),
            bpm: 150.0,
            ppqn: 480,
            current_tick: Tick::zero(),
            running: Arc::new(Mutex::new(false)),
        }
    }

    fn tick_duration(&self) -> u64 {
        let beat_duration = 60.0 / self.bpm;
        let tick_duration = (beat_duration / self.ppqn as f64) * 1000.0;
        let duration = (tick_duration * 1000.0) as u64;
        duration
    }

    pub async fn process(
        mut self,
        mut command_receiver: tokio::sync::mpsc::UnboundedReceiver<SequencerCommand>,
    ) -> ! {
        tracing::info!("sequencer listening for commands");
        let mut interval = time::interval(Duration::from_micros(self.tick_duration()));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if *self.running.lock().unwrap() {
                        self.process_events();
                        self.current_tick.tick();
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

    fn stop(&mut self) {
        *self.running.lock().unwrap() = false;

        self.midi_engine_sender
            .as_mut()
            .map(|sender| sender.send((MidiMessage::GlobalNoteOff, 0, 0)).unwrap());
        self.midi_engine_sender
            .as_mut()
            .map(|sender| sender.send((MidiMessage::GlobalNoteOff, 0, 0)).unwrap());
    }

    fn play(&mut self) {
        *self.running.lock().unwrap() = true;
    }

    fn process_events(&mut self) {
        let tracks = &self
            .data_layer
            .lock()
            .unwrap()
            .project_manager
            .track_manager
            .tracks;

        for track in tracks {
            if let Some(event_entry) = track.event_list.get(&self.current_tick) {
                let event = event_entry.event.clone();
                tracing::info!("{} - {}", track, event);

                if event_entry.active {
                    let message = event.get_message();
                    let instrument = &track.instrument;
                    self.midi_engine_sender
                        .as_mut()
                        .map(|sender| sender.send((message, instrument.port, instrument.channel)));
                }
            }
        }
    }
}
