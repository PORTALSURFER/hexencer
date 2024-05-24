use hexencer_core::{data::DataLayer, Instrument, MidiEvent, MidiMessage, Note};
use midir::MidiOutput;
use std::{sync::Arc, sync::Mutex, time::Duration};
use tokio::{task, time};

pub enum SequencerCommand {
    Play,
    Stop,
}
type MidiEngineSender = tokio::sync::mpsc::UnboundedSender<MidiMessage>;

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
        let (midi_engine_sender, midi_engine_receiver) = tokio::sync::mpsc::unbounded_channel();

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

    pub async fn listen(
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
                        let midi_message = MidiMessage::NoteOn(
                            Note::new(66, 1, 64, 50),
                            Instrument::new("first", 0),
                        );
                        let event1 = MidiEvent::new(0, midi_message, false);
                        let midi_message = MidiMessage::NoteOn(
                            Note::new(66, 1, 64, 50),
                            Instrument::new("first", 0),
                        );
                        let event2 = MidiEvent::new(480, midi_message, false);
                        let midi_message = MidiMessage::NoteOn(
                            Note::new(66, 1, 64, 50),
                            Instrument::new("first", 0),
                        );
                        let event3 = MidiEvent::new(960, midi_message, false);
                        let events = vec![event1, event2, event3];

                        let current_events: Vec<&MidiEvent> = events.iter().filter(|event| event.tick == self.current_tick).collect();

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

    fn play_events(&self, current_tick: u64, events: Vec<&MidiEvent>) {
        for event in events {
            println!("playing event {:?} {:?}", current_tick, event);
        }
    }
}

#[derive(Default)]
pub struct MidiEngine {
    conn_out: Option<midir::MidiOutputConnection>,
    conn_out2: Option<midir::MidiOutputConnection>,
    running: bool,
}

impl MidiEngine {
    pub fn new() -> Self {
        let midi_out = MidiOutput::new("Test Output").unwrap();
        let midi_out2 = MidiOutput::new("Test Output").unwrap();
        // let midi_out2 = MidiOutput::new("Test Output").unwrap();

        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_ports2 = midi_out2.ports();

        let port = out_ports.get(4).ok_or("no output port found").unwrap();
        let port2 = out_ports2.get(3).ok_or("no output port found").unwrap();

        println!("\nOpening connection");
        let conn_out = Some(midi_out.connect(port, "midir-test").unwrap());
        let conn_out2 = Some(midi_out2.connect(port2, "midir-test2").unwrap());

        Self {
            conn_out,
            conn_out2,
            running: false,
        }
    }

    async fn play(&mut self, note: &Note, instrument: &Instrument) {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        // We're ignoring errors in here

        match instrument.midi_port {
            1 => {
                let _ = self
                    .conn_out
                    .as_mut()
                    .map(|s| s.send(&[NOTE_ON_MSG, note.index, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(note.duration * 1000)).await;
                let _ = self
                    .conn_out
                    .as_mut()
                    .map(|s| s.send(&[NOTE_OFF_MSG, note.index, VELOCITY]));
            }
            2 => {
                let _ = self
                    .conn_out2
                    .as_mut()
                    .map(|s| s.send(&[NOTE_ON_MSG, note.index, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(note.duration * 1000)).await;
                let _ = self
                    .conn_out2
                    .as_mut()
                    .map(|s| s.send(&[NOTE_OFF_MSG, note.index, VELOCITY]));
            }
            _ => {}
        }
    }

    fn stop(&mut self) {
        println!("\nClosing connection");
        self.conn_out.take().map(|c| c.close());
        println!("Connection closed");
    }

    pub async fn init(mut self, mut rx: tokio::sync::mpsc::UnboundedReceiver<MidiMessage>) {
        println!("running midiio");
        while let Some(v) = rx.recv().await {
            match v {
                MidiMessage::NoteOn(note, instrument) => {
                    println!("playing");
                    self.play(&note, &instrument).await;
                }
                MidiMessage::NoteOff(note) => {
                    println!("stopping");
                    self.stop();
                }
            }
        }
        println!("done");
    }
}
